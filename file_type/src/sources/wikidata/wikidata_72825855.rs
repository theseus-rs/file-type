use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_72825855: FileType = FileType {
    file_format: &FileFormat {
        id: 72_825_855,
        source_type: SourceType::Wikidata,
        name: "OpenCanvas Image",
        extensions: &["oci"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
