use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117035677: FileType = FileType {
    file_format: &FileFormat {
        id: 117_035_677,
        source_type: SourceType::Wikidata,
        name: "TurboCAD for Windows Metafile",
        extensions: &["wmf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
