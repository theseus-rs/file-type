use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1075962: FileType = FileType {
    file_format: &FileFormat {
        id: 1_075_962,
        source_type: SourceType::Wikidata,
        name: "RealMedia",
        extensions: &["rm", "rv"],
        media_types: &["application/vnd.rn-realmedia"],
        signatures: &[],
        related_formats: &[],
    },
};
