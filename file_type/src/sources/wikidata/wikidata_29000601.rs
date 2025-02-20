use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29000601: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_601,
        source_type: SourceType::Wikidata,
        name: "Patch Storage File",
        extensions: &["psf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
