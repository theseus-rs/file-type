use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128597179: FileType = FileType {
    file_format: &FileFormat {
        id: 128_597_179,
        source_type: SourceType::Wikidata,
        name: "AMDGPU file",
        extensions: &["isa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
