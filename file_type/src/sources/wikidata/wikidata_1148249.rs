use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1148249: FileType = FileType {
    file_format: &FileFormat {
        id: 1_148_249,
        source_type: SourceType::Wikidata,
        name: "H.261",
        extensions: &["h261"],
        media_types: &["video/H261"],
        signatures: &[],
        related_formats: &[],
    },
};
