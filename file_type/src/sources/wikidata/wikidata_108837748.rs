use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_108837748: FileType = FileType {
    file_format: &FileFormat {
        id: 108_837_748,
        source_type: SourceType::Wikidata,
        name: "OmniPage Pro Document 11",
        extensions: &["opd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
