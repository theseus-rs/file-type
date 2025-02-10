use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125041640: FileType = FileType {
    file_format: &FileFormat {
        id: 125_041_640,
        source_type: SourceType::Wikidata,
        name: "Yoshimi Instrument File",
        extensions: &["xiy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
