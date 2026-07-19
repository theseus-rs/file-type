use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137900621: FileType = FileType {
    file_format: &FileFormat {
        id: 137_900_621,
        source_type: SourceType::Wikidata,
        name: "Ngspice circuit file",
        extensions: &["cir"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
