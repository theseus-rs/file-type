use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28756037: FileType = FileType {
    file_format: &FileFormat {
        id: 28_756_037,
        source_type: SourceType::Wikidata,
        name: "FON",
        extensions: &["fon"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
