use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61964244: FileType = FileType {
    file_format: &FileFormat {
        id: 61_964_244,
        source_type: SourceType::Wikidata,
        name: "pulse EKKO header file",
        extensions: &["hd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
