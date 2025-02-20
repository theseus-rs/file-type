use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130386156: FileType = FileType {
    file_format: &FileFormat {
        id: 130_386_156,
        source_type: SourceType::Wikidata,
        name: "Nit source code file",
        extensions: &["nit"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
