use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205397: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_397,
        source_type: SourceType::Wikidata,
        name: "Nikon Electronic Format (Nikon D1 variant)",
        extensions: &["nef"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
