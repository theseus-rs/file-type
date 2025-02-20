use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2104918: FileType = FileType {
    file_format: &FileFormat {
        id: 2_104_918,
        source_type: SourceType::Wikidata,
        name: "Portable Sound Format",
        extensions: &["minipsf", "psf", "psflib"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
