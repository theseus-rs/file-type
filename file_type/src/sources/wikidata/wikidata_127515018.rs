use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127515018: FileType = FileType {
    file_format: &FileFormat {
        id: 127_515_018,
        source_type: SourceType::Wikidata,
        name: "Typescript implementation file",
        extensions: &["ts"],
        media_types: &["application/x-typescript", "text/x-typescript"],
        signatures: &[],
        related_formats: &[],
    },
};
