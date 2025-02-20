use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111721061: FileType = FileType {
    file_format: &FileFormat {
        id: 111_721_061,
        source_type: SourceType::Wikidata,
        name: "Free-format Fortran 90 source",
        extensions: &["f90"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
