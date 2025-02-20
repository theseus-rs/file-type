use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27966927: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_927,
        source_type: SourceType::Wikidata,
        name: "PSF2",
        extensions: &["minipsf2", "psf2", "psf2lib"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
