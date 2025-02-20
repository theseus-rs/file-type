use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2307314: FileType = FileType {
    file_format: &FileFormat {
        id: 2_307_314,
        source_type: SourceType::Wikidata,
        name: "Direct Access Archive",
        extensions: &["daa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
