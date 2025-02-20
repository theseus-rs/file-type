use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117401724: FileType = FileType {
    file_format: &FileFormat {
        id: 117_401_724,
        source_type: SourceType::Wikidata,
        name: "NBT",
        extensions: &["dat", "nbt"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
