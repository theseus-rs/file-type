use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27350170: FileType = FileType {
    file_format: &FileFormat {
        id: 27_350_170,
        source_type: SourceType::Wikidata,
        name: "ADRG Transmittal Header File",
        extensions: &["thf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
