use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111496643: FileType = FileType {
    file_format: &FileFormat {
        id: 111_496_643,
        source_type: SourceType::Wikidata,
        name: "Spectrum 512 Extended, version 2",
        extensions: &["spx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
