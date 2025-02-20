use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600231: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_231,
        source_type: SourceType::Wikidata,
        name: "APW",
        extensions: &["apw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
