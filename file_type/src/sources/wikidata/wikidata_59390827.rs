use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59390827: FileType = FileType {
    file_format: &FileFormat {
        id: 59_390_827,
        source_type: SourceType::Wikidata,
        name: "Domino XML Document Export",
        extensions: &["dxl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
