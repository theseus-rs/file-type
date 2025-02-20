use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
