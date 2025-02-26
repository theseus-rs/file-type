use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132641591: FileType = FileType {
    file_format: &FileFormat {
        id: 132_641_591,
        source_type: SourceType::Wikidata,
        name: "Combined package specification plus body file",
        extensions: &["pck"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
