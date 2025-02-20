use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
