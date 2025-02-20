use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_291: FileType = FileType {
    file_format: &FileFormat {
        id: 291,
        source_type: SourceType::Linguist,
        name: "PostScript",
        extensions: &["eps", "epsi", "pfa", "ps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
