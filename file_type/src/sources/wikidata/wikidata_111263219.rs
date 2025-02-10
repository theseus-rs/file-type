use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111263219: FileType = FileType {
    file_format: &FileFormat {
        id: 111_263_219,
        source_type: SourceType::Wikidata,
        name: "DCM module",
        extensions: &["dcm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
