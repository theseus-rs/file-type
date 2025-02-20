use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1027477: FileType = FileType {
    file_format: &FileFormat {
        id: 1_027_477,
        source_type: SourceType::Wikidata,
        name: "Caltech Intermediate Form",
        extensions: &["cif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
