use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125925206: FileType = FileType {
    file_format: &FileFormat {
        id: 125_925_206,
        source_type: SourceType::Wikidata,
        name: "Papyrus Author database",
        extensions: &["pb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
