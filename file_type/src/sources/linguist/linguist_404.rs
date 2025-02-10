use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_404: FileType = FileType {
    file_format: &FileFormat {
        id: 404,
        source_type: SourceType::Linguist,
        name: "XSLT",
        extensions: &["xsl", "xslt"],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
