use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27966948: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_948,
        source_type: SourceType::Wikidata,
        name: "SPC",
        extensions: &["rsn", "spc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
