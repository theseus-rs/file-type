use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111009733: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_733,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Note Card File format",
        extensions: &["not"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
