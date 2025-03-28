use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127411070: FileType = FileType {
    file_format: &FileFormat {
        id: 127_411_070,
        source_type: SourceType::Wikidata,
        name: "Nim source code file",
        extensions: &["nim", "nimrod"],
        media_types: &["text/x-nim"],
        signatures: &[],
        related_formats: &[],
    },
};
