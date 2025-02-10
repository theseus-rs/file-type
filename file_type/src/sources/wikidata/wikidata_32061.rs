use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_32061: FileType = FileType {
    file_format: &FileFormat {
        id: 32_061,
        source_type: SourceType::Wikidata,
        name: "XSL",
        extensions: &["xsl", "xslt"],
        media_types: &["application/xslt+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
