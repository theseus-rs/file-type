use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130543516: FileType = FileType {
    file_format: &FileFormat {
        id: 130_543_516,
        source_type: SourceType::Wikidata,
        name: "PyPy log file format",
        extensions: &["pypylog"],
        media_types: &["application/x-pypylog"],
        signatures: &[],
        related_formats: &[],
    },
};
