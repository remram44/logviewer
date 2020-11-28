Log Viewer
==========

This is a web viewer application, written in Rust (server) and JavaScript (frontend). It can process log files to filter lines matching a pattern, mark lines of interest in different colors (warnings, errors, ...), and extract information (client IP, HTTP code, ...) via regex captures.

It is born from the realization that log viewers are not very well suited for the quick analysis of specific problems of patterns. They mainly fall into two categories:

* Standalone viewers: those are often desktop applications. They can quickly open a log file, but are often limited to a few formats (e.g. HTTP server logs) and often can't do much more filtering than applying a single regex at a time.
* Full-fledged log collection applications: an example of this is Kibana. While this can extract fields from logs and index them for powerful filtering and aggregation features, this extraction has to be configured ahead of time, making it difficult to quickly inspect a log file or getting started with an unknown format.

Furthermore, this is being developed as a single-binary web server, so that you can easily run it on the server where you are storing the logs, and access the interface in your browser over an SSH tunnel.

Current status
--------------

The filtering/processing engine works, next step is the web interface.

Filters
-------

A query over log records is expressed as a list of operations. An operation can be a condition, allowing you to process records separately depending on how they match a regex.

For example, you can extract different fields from log lines coming from different web services, and hide one service entirely (or records of a specific level from a specific service where the client matches something, etc. The sky's the limit!).

This is the example query that I am building from. The idea is that it wouldn't have to be entered, but would be input through a graphical interface, and sent to the server as JSON:

```
service.log
    IF record match "^(?P<time>[0-9TZ-]+) (?P<message>.*)$"
        IF message match "^(?P<client>[0-9]+(\\.[0-9]+){3}) ([^ ]+ ){2}\\[.+\\] \"(?P<vhost>[^\"]+)\""
            SET service = frontend
        ELIF message match "^service=(?P<service>.+)\b"
            NOP
        COLOR-BY service
    ELSE
        SET time = last time
    IF record match "\bERROR\b"
        SET error   (= "")
    ELIF record match "\bDEBUG\b"
        SKIP
```
