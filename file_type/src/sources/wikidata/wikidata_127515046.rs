use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127515046: FileType = FileType {
    file_format: &FileFormat {
        id: 127_515_046,
        source_type: SourceType::Wikidata,
        name: "Typescript declaration file",
        extensions: &["d.ts"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
