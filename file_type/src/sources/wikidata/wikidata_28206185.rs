use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206185: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_185,
        source_type: SourceType::Wikidata,
        name: "GIMP Pattern",
        extensions: &["pat"],
        media_types: &["image/x-gimp-pat"],
        signatures: &[],
        related_formats: &[],
    },
};
