use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111272306: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_306,
        source_type: SourceType::Wikidata,
        name: "Ensoniq EPS instrument file",
        extensions: &["efe"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
