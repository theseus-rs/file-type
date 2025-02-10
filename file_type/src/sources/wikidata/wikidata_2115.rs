use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2115: FileType = FileType {
    file_format: &FileFormat {
        id: 2_115,
        source_type: SourceType::Wikidata,
        name: "XML",
        extensions: &["xml"],
        media_types: &["application/xml", "text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
