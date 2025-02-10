use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_73513552: FileType = FileType {
    file_format: &FileFormat {
        id: 73_513_552,
        source_type: SourceType::Wikidata,
        name: "Puppy Linux DotPup installer package",
        extensions: &["pup"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
