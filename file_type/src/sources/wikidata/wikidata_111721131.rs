use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111721131: FileType = FileType {
    file_format: &FileFormat {
        id: 111_721_131,
        source_type: SourceType::Wikidata,
        name: "Fixed-format Fortran source",
        extensions: &["f", "f77", "for"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
