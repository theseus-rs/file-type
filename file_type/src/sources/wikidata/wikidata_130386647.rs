use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130386647: FileType = FileType {
    file_format: &FileFormat {
        id: 130_386_647,
        source_type: SourceType::Wikidata,
        name: "NuSMV file format",
        extensions: &["smv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
