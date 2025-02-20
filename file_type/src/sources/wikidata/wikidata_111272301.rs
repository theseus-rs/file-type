use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111272301: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_301,
        source_type: SourceType::Wikidata,
        name: "Ensoniq ASR instrument file",
        extensions: &["efa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
