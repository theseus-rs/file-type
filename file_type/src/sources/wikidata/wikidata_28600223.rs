use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600223: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_223,
        source_type: SourceType::Wikidata,
        name: "APT",
        extensions: &["apt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
