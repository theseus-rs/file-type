use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206535: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_535,
        source_type: SourceType::Wikidata,
        name: "Magick Persistent Cache (.mpc file)",
        extensions: &["mpc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
