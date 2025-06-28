use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134715818: FileType = FileType {
    file_format: &FileFormat {
        id: 134_715_818,
        source_type: SourceType::Wikidata,
        name: "Seed7 source code file",
        extensions: &["s7i"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
