use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_146: FileType = FileType {
    file_format: &FileFormat {
        id: 146,
        source_type: SourceType::Linguist,
        name: "HTML",
        extensions: &["hta", "htm", "html", "html.hl", "inc", "xht", "xhtml"],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
