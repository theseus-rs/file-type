use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
