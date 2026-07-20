use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137705310: FileType = FileType {
    file_format: &FileFormat {
        id: 137_705_310,
        source_type: SourceType::Wikidata,
        name: "PO file",
        extensions: &["po"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
