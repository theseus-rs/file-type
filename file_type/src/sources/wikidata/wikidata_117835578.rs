use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117835578: FileType = FileType {
    file_format: &FileFormat {
        id: 117_835_578,
        source_type: SourceType::Wikidata,
        name: "DataBeam file",
        extensions: &["dbx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
