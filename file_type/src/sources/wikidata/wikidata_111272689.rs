use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111272689: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_689,
        source_type: SourceType::Wikidata,
        name: "Farandoyle linear module format",
        extensions: &["f2r"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
