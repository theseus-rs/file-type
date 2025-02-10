use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28757979: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_979,
        source_type: SourceType::Wikidata,
        name: "Windows Setup inf_loc file",
        extensions: &["inf_loc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
