use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_21620033: FileType = FileType {
    file_format: &FileFormat {
        id: 21_620_033,
        source_type: SourceType::Wikidata,
        name: "XDMF",
        extensions: &["xdmf", "xmf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
