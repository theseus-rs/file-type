use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207000: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_000,
        source_type: SourceType::Wikidata,
        name: "Picture Packer",
        extensions: &["pp1", "pp2", "pp3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
