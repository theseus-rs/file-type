use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137786800: FileType = FileType {
    file_format: &FileFormat {
        id: 137_786_800,
        source_type: SourceType::Wikidata,
        name: "DxO ONE SuperRAW format",
        extensions: &["dxo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
