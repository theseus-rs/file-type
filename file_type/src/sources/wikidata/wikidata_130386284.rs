use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130386284: FileType = FileType {
    file_format: &FileFormat {
        id: 130_386_284,
        source_type: SourceType::Wikidata,
        name: "Nix file format",
        extensions: &["nix"],
        media_types: &["text/x-nix"],
        signatures: &[],
        related_formats: &[],
    },
};
