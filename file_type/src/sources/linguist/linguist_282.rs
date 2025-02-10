use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_282: FileType = FileType {
    file_format: &FileFormat {
        id: 282,
        source_type: SourceType::Linguist,
        name: "Perl",
        extensions: &[
            "al", "cgi", "fcgi", "perl", "ph", "pl", "plx", "pm", "psgi", "t",
        ],
        media_types: &["text/x-perl"],
        signatures: &[],
        related_formats: &[],
    },
};
