use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111272308: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_308,
        source_type: SourceType::Wikidata,
        name: "Ensoniq KT instrument file",
        extensions: &["efk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
