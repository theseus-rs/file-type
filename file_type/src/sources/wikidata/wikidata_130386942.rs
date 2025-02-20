use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130386942: FileType = FileType {
    file_format: &FileFormat {
        id: 130_386_942,
        source_type: SourceType::Wikidata,
        name: "objdump file format",
        extensions: &["objdump"],
        media_types: &["text/x-objdump"],
        signatures: &[],
        related_formats: &[],
    },
};
