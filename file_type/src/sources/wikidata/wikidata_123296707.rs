use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123296707: FileType = FileType {
    file_format: &FileFormat {
        id: 123_296_707,
        source_type: SourceType::Wikidata,
        name: "CD-Face Layout",
        extensions: &["ntp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
