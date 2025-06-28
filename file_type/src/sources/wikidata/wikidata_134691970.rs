use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134691970: FileType = FileType {
    file_format: &FileFormat {
        id: 134_691_970,
        source_type: SourceType::Wikidata,
        name: "NooJ inflectional/derivational morphological grammar file",
        extensions: &["nof"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
