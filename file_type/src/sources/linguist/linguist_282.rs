use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_282: FileFormat = FileFormat {
    id: 282,
    source_type: SourceType::Linguist,
    name: "Perl",
    extensions: &[
        "al", "cgi", "fcgi", "perl", "ph", "pl", "plx", "pm", "psgi", "t",
    ],
    media_types: &["text/x-perl"],
    signatures: &[],
    related_formats: &[],
};
