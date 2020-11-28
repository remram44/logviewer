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
