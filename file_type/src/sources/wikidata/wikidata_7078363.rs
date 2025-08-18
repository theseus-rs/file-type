use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7078363: FileType = FileType {
    file_format: &FileFormat {
        id: 7_078_363,
        source_type: SourceType::Wikidata,
        name: "ODTTF",
        extensions: &["odttf"],
        media_types: &["application/vnd.openxmlformats-officedocument.obfuscatedFont"],
        signatures: &[],
        related_formats: &[],
    },
};
