use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_96081191: FileType = FileType {
    file_format: &FileFormat {
        id: 96_081_191,
        source_type: SourceType::Wikidata,
        name: "SystemModeler experiment format",
        extensions: &["sme"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
