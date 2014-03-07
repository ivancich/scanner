#[warn(non_camel_case_types)];
#[allow(dead_code)];

#[ crate_id = "github.com/ivancich/scanner#0.1.3" ];
#[ crate_type = "lib" ];

#[ desc = "A character stream scanner to make processing input easier." ];
#[ license = "BSD" ];
#[ comment = "Empty comment for now." ];

use std::io::Reader;
use std::io::buffered::BufferedReader;
use std::char::is_whitespace;
use std::char::is_digit;
use std::char::to_digit;
use std::option::Option;

#[cfg(test)]
mod test;

struct CharStream<R> {
    buffer : ~BufferedReader<R>,
    last_char : Option<char>,
    last_is_filled : bool,
}

impl<R:Reader> CharStream<R> {
    fn new(buffer_p : ~BufferedReader<R>) -> ~CharStream<R> {
        ~CharStream{ buffer : buffer_p,
                     last_char : None,
                     last_is_filled : false, }
    }
    
    fn peek(&mut self) -> Option<char> {
        if ! self.last_is_filled {
            self.last_char = self.buffer.read_char();
            self.last_is_filled = true;
        }
        debug!("peek returning {}", self.last_char);
        self.last_char
    }

    fn push(&mut self, c : Option<char>) {
        assert!(! self.last_is_filled);
        self.last_char = c;
        self.last_is_filled = true;
        debug!("pushing {}", self.last_char);
    }

    fn next(&mut self) -> Option<char> {
        if self.last_is_filled {
            debug!("next returning {}", self.last_char);
            self.last_is_filled = false;
            self.last_char
        } else {
            let ch = self.buffer.read_char();
            debug!("next returning {}", ch);
            ch
        }
    }
}


pub struct Scanner<R> {
    stream : ~CharStream<R>
}


impl<R:Reader> Scanner<R> {
    pub fn new_from_buffered_reader(buffer : ~BufferedReader<R>)
                                    -> ~Scanner<R> {
        let stream_v : ~CharStream<R> = CharStream::new(buffer);
        ~Scanner{ stream : stream_v }
    }

    pub fn new_from_reader(reader : R) -> ~Scanner<R> {
        let buffer = BufferedReader::new(reader);
        Scanner::new_from_buffered_reader(~buffer)
    }

    pub fn skip_white(&mut self) {
        loop {
            let ch = self.stream.peek();
            match ch {
                None =>
                    return,
                Some(c) if ! is_whitespace(c) =>
                    return,
                _ =>
                    { self.stream.next(); }
            }
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
        return self.stream.next();
    }

    pub fn next_uint(&mut self) -> Option<uint> {
        self.skip_white();
        self.next_uint_help()
    }

    fn next_uint_help(&mut self) -> Option<uint> {
        let mut accum : uint = 0;
        let mut found_digit = false;
        let mut ch = self.stream.next();
        while ch.is_some() && is_digit(ch.unwrap()) {
            found_digit = true;
            accum = accum * 10 + to_digit(ch.unwrap(), 10).unwrap();
            ch = self.stream.next();
        }
        self.stream.push(ch);

        if found_digit {
            Some(accum)
        } else {
            None
        }
    }

    pub fn next_int(&mut self) -> Option<int> { 
        self.skip_white();
        self.next_int_help()
    }

    fn next_int_help(&mut self) -> Option<int> { 
        let mut sign = 1;

        let ch = self.stream.next();
        match ch {
            None => return None,
            Some(c) if (c == '+') =>
                sign = 1,
            Some(c) if (c == '-') =>
                sign = -1,
            Some(c) if is_digit(c) =>
                self.stream.push(ch),
            _ =>
                return None
        }

        let unsigned_value = self.next_uint_help();
        match unsigned_value {
            None => None,
            Some(value) => Some(value as int * sign)
        }
    }

}
