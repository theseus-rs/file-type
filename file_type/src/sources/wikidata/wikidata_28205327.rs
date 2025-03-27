use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205327: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_327,
        source_type: SourceType::Wikidata,
        name: "CHDK raw",
        extensions: &["crw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
