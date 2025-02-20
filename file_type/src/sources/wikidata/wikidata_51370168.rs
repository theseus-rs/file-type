use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51370168: FileType = FileType {
    file_format: &FileFormat {
        id: 51_370_168,
        source_type: SourceType::Wikidata,
        name: "Postscript Support File",
        extensions: &["psf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
