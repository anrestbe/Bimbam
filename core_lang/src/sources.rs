//! This module tracks source files and metadata.

use generational_arena::{Arena, Index};
use lazy_static::lazy_static;
use std::{path::PathBuf, sync::Mutex};

lazy_static! {
    /// Tracks all source code files and metadata associated with them.
    pub static ref SOURCES: Arena<SourceFile> = Default::default();
}

/// Represents a Sway source code file.
pub struct SourceFile {
    /// The absolute path to the file.
    pub file_path: PathBuf,
    /// The one and only copy in memory of this file's contents as a string.
    /// Only references to this should be distributed to avoid over-cloning.
    pub file_content: String,
}

/// Represents a span of a specific section of source code in a specific file.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Span {
    arena_idx: Index,
    start: usize,
    end: usize,
}

impl Span {
    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn start_pos(&self) -> pest::Position {
        let input_file: &SourceFile = SOURCES.get(self.arena_idx).expect("infallible");
        pest::Position::new(&input_file.file_content, self.start).expect("infallible")
    }

    pub fn end_pos<'a>(&self) -> pest::Position<'a> {
        let input_file: &SourceFile = SOURCES.get(self.arena_idx).expect("infallible");
        pest::Position::new(&input_file.file_content, self.end).expect("infallible")
    }

    pub fn split<'a>(&self) -> (pest::Position<'a>, pest::Position<'a>) {
        let input_file: &SourceFile = SOURCES.get(self.arena_idx).expect("infallible");
        (
            pest::Position::new(&input_file.file_content, self.start).expect("infallible"),
            pest::Position::new(&input_file.file_content, self.end).expect("infallible"),
        )
    }

    pub fn as_str<'a>(&self) -> &'a str {
        let input_file: &SourceFile = SOURCES.get(self.arena_idx).expect("infallible");
        &input_file.file_content[self.start..self.end]
    }

    pub fn input(&self) -> &str {
        let input_file: &SourceFile = SOURCES.get(self.arena_idx).expect("infallible");
        input_file.file_content.as_str()
    }

    pub fn path(&self) -> String {
        let input_file: &SourceFile = SOURCES.get(self.arena_idx).expect("infallible");
        input_file
            .file_path
            .clone()
            .into_os_string()
            .into_string()
            .expect("hopefully the file name isn't invalid utf-8")
    }
}
