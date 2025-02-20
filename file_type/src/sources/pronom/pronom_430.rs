use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_430: FileType = FileType {
    file_format: &FileFormat {
        id: 430,
        source_type: SourceType::Pronom,
        name: "Extensible Stylesheet Language",
        extensions: &["xsl"],
        media_types: &["application/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
