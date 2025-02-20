use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111431164: FileType = FileType {
    file_format: &FileFormat {
        id: 111_431_164,
        source_type: SourceType::Wikidata,
        name: "C source code file",
        extensions: &["c"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
