use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129423705: FileType = FileType {
    file_format: &FileFormat {
        id: 129_423_705,
        source_type: SourceType::Wikidata,
        name: "Gosu source code file",
        extensions: &["gs"],
        media_types: &["text/x-gosu"],
        signatures: &[],
        related_formats: &[],
    },
};
