use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127116930: FileType = FileType {
    file_format: &FileFormat {
        id: 127_116_930,
        source_type: SourceType::Wikidata,
        name: "IDLSAV file",
        extensions: &["sav"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
