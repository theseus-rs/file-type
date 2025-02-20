use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
