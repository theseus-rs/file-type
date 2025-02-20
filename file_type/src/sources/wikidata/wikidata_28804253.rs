use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28804253: FileType = FileType {
    file_format: &FileFormat {
        id: 28_804_253,
        source_type: SourceType::Wikidata,
        name: "Uniform Office Format",
        extensions: &["eof"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
