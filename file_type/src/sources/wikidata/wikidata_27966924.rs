use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27966924: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_924,
        source_type: SourceType::Wikidata,
        name: "PSF1",
        extensions: &["minipsf", "minipsf1", "psf", "psf1", "psf1lib", "psflib"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
