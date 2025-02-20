use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205711: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_711,
        source_type: SourceType::Wikidata,
        name: "STW",
        extensions: &["stw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
