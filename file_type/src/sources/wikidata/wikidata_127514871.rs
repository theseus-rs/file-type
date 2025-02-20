use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127514871: FileType = FileType {
    file_format: &FileFormat {
        id: 127_514_871,
        source_type: SourceType::Wikidata,
        name: "Text2tags file",
        extensions: &["t2t"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
