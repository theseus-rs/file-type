use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111721108: FileType = FileType {
    file_format: &FileFormat {
        id: 111_721_108,
        source_type: SourceType::Wikidata,
        name: "Free-format Fortran 95 source",
        extensions: &["f95"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
