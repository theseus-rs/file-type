use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205901: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_901,
        source_type: SourceType::Wikidata,
        name: "DGI",
        extensions: &["dgi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
