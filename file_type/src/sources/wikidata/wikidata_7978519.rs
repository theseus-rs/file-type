use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7978519: FileType = FileType {
    file_format: &FileFormat {
        id: 7_978_519,
        source_type: SourceType::Wikidata,
        name: "Web Integration Compound Document",
        extensions: &[],
        media_types: &["application/cdf+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
