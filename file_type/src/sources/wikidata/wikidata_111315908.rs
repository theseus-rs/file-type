use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111315908: FileType = FileType {
    file_format: &FileFormat {
        id: 111_315_908,
        source_type: SourceType::Wikidata,
        name: "INRS-Telecommunications audio file",
        extensions: &["inrs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
