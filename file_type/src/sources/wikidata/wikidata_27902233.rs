use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27902233: FileType = FileType {
    file_format: &FileFormat {
        id: 27_902_233,
        source_type: SourceType::Wikidata,
        name: "Amateur Data Interchange Format, ADX variant, version 3.0.3",
        extensions: &["adx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
