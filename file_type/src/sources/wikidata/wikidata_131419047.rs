use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131419047: FileType = FileType {
    file_format: &FileFormat {
        id: 131_419_047,
        source_type: SourceType::Wikidata,
        name: "WebGPU Shading Language file format",
        extensions: &["wgsl"],
        media_types: &["text/wgsl"],
        signatures: &[],
        related_formats: &[],
    },
};
