use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134291928: FileType = FileType {
    file_format: &FileFormat {
        id: 134_291_928,
        source_type: SourceType::Wikidata,
        name: "Clipper script file",
        extensions: &["clp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
