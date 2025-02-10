use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_72175258: FileType = FileType {
    file_format: &FileFormat {
        id: 72_175_258,
        source_type: SourceType::Wikidata,
        name: "Kaspersky Anti-Virus signature bases",
        extensions: &["kdc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
