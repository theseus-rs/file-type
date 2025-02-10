use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_52005598: FileType = FileType {
    file_format: &FileFormat {
        id: 52_005_598,
        source_type: SourceType::Wikidata,
        name: "AMI Draw Vector Image",
        extensions: &["sdw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
