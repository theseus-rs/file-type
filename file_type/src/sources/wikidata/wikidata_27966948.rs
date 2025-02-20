use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
