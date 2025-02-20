use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5009675: FileType = FileType {
    file_format: &FileFormat {
        id: 5_009_675,
        source_type: SourceType::Wikidata,
        name: "CCP4",
        extensions: &["ccp4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
