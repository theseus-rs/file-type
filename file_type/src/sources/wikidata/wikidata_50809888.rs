use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50809888: FileType = FileType {
    file_format: &FileFormat {
        id: 50_809_888,
        source_type: SourceType::Wikidata,
        name: "Google Document Link File",
        extensions: &[
            "gdoc", "gdraw", "gform", "gmap", "gsheet", "gsite", "gslides",
        ],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
