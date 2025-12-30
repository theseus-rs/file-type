use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_247918769: FileType = FileType {
    file_format: &FileFormat {
        id: 247_918_769,
        source_type: SourceType::Linguist,
        name: "Go Template",
        extensions: &["gohtml", "gotmpl", "html.tmpl", "tmpl", "tpl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
