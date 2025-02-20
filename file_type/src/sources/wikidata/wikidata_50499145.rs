use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50499145: FileType = FileType {
    file_format: &FileFormat {
        id: 50_499_145,
        source_type: SourceType::Wikidata,
        name: "QuickDraw 3D Metafile (ASCII)",
        extensions: &["3dmf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
