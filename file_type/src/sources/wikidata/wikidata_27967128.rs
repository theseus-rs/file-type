use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967128: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_128,
        source_type: SourceType::Wikidata,
        name: "DMC",
        extensions: &["dmc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
