use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207363: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_363,
        source_type: SourceType::Wikidata,
        name: "TealPaint PDB",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
