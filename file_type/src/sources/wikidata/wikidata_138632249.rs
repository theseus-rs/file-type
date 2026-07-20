use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138632249: FileType = FileType {
    file_format: &FileFormat {
        id: 138_632_249,
        source_type: SourceType::Wikidata,
        name: "LayoutEditor Cloud file format",
        extensions: &["lec"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
