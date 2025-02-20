use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126951686: FileType = FileType {
    file_format: &FileFormat {
        id: 126_951_686,
        source_type: SourceType::Wikidata,
        name: "Nemerle source code file",
        extensions: &["n"],
        media_types: &["text/x-nemerle"],
        signatures: &[],
        related_formats: &[],
    },
};
