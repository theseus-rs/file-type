use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967086: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_086,
        source_type: SourceType::Wikidata,
        name: "Soundfactory",
        extensions: &["psf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
