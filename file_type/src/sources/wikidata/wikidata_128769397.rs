use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128769397: FileType = FileType {
    file_format: &FileFormat {
        id: 128_769_397,
        source_type: SourceType::Wikidata,
        name: "Concise Data Definition Language file",
        extensions: &["cddl"],
        media_types: &["text/x-cddl"],
        signatures: &[],
        related_formats: &[],
    },
};
