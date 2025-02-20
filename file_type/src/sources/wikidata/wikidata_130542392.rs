use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130542392: FileType = FileType {
    file_format: &FileFormat {
        id: 130_542_392,
        source_type: SourceType::Wikidata,
        name: "Parallel Thread Execution file format",
        extensions: &["ptx"],
        media_types: &["text/x-ptx"],
        signatures: &[],
        related_formats: &[],
    },
};
