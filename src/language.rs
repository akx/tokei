// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

use std::fmt;

#[derive(Debug)]
pub struct Language<'a> {
	pub name: &'a str,
	pub line_comment: &'a str,
	pub multi_line: &'a str,
	pub multi_line_end: &'a str,
	pub files: Vec<String>,
	pub code: usize,
	pub comments: usize,
	pub blanks: usize,
	pub lines: usize,
	pub total: usize,
	pub printed: bool,
}


impl<'a> Language<'a> {
    pub fn new<'b>(name: &'a str,
        line_comment: &'a str,
        multi_line: &'a str,
        multi_line_end: &'a str) -> Language<'a> {

		Language {
			name: name,
			line_comment: line_comment,
			multi_line: multi_line,
			multi_line_end: multi_line_end,
			files: Vec::new(),
			code: 0,
			comments: 0,
			blanks: 0,
			lines: 0,
			total: 0,
			printed: false,
		}
	}

	pub fn new_c(name: &'a str) -> Language<'a> {
		Language {
			name: name,
			line_comment: "//",
			multi_line: "/*",
			multi_line_end: "*/",
			files: Vec::new(),
			code: 0,
			comments: 0,
			blanks: 0,
			lines: 0,
			total: 0,
			printed: false,
		}
	}

	pub fn new_html(name: &'a str) -> Language<'a> {
		Language {
			name: name,
			line_comment: "<!--",
			multi_line: "<!--",
			multi_line_end: "-->",
			files: Vec::new(),
			code: 0,
			comments: 0,
			blanks: 0,
			lines: 0,
			total: 0,
			printed: false,
		}
	}

	pub fn new_blank(name: &'a str) -> Language<'a> {
		Language {
			name: name,
			line_comment: "",
			multi_line: "",
			multi_line_end: "",
			files: Vec::new(),
			code: 0,
			comments: 0,
			blanks: 0,
			lines: 0,
			total: 0,
			printed: false,
		}
	}

	pub fn new_single(name: &'a str, line_comment: &'a str) -> Language<'a> {
		Language {
			name: name,
			line_comment: line_comment,
			multi_line: "",
			multi_line_end: "",
			files: Vec::new(),
			code: 0,
			comments: 0,
			blanks: 0,
			lines: 0,
			total: 0,
			printed: false,
		}
	}

	pub fn is_empty(&self) -> bool {
		self.code == 0 && self.comments == 0 && self.blanks == 0 && self.lines == 0
	}

	pub fn add_code(&mut self, num: usize) {
	    self.code += num;
	}

	pub fn add_comments(&mut self, num: usize) {
	    self.comments += num;
	}

	pub fn add_blanks(&mut self, num: usize) {
	    self.blanks += num;
	}

	pub fn add_lines(&mut self, num: usize) {
	    self.lines += num;
	}

	pub fn add_total(&mut self, num: usize) {
	    self.total += num;
	}

	pub fn printed(&mut self, printed: bool) {
	    self.printed = printed;
	}
}

impl<'a> fmt::Display for Language<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let total = if self.total == 0 {
            self.files.len()
        } else {
            self.total
        };
        write!(f," {: <15} {: >15} {:>15} {:>15} {:>15} {:>15}", self.name, total, self.lines, self.blanks, self.comments, self.code)
    }
}
