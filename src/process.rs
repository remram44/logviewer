use crate::filters::View;
use crate::readers::LogReader;

pub struct FilteredLogIterator<R: LogReader> {
    reader: R,
    view: View,
}

impl<R: LogReader> Iterator for FilteredLogIterator<R> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        todo!() // filter
    }
}

pub fn process<R: LogReader>(reader: R, view: View) -> FilteredLogIterator<R> {
    FilteredLogIterator { reader, view }
}
