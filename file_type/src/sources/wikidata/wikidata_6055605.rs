use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_6055605: FileType = FileType {
    file_format: &FileFormat {
        id: 6_055_605,
        source_type: SourceType::Wikidata,
        name: "Internet Open Trading Protocol",
        extensions: &[],
        media_types: &["application/IOTP"],
        signatures: &[],
        related_formats: &[],
    },
};
