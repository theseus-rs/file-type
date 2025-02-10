use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131293815: FileType = FileType {
    file_format: &FileFormat {
        id: 131_293_815,
        source_type: SourceType::Wikidata,
        name: "Tera Term macro source code file",
        extensions: &["ttl"],
        media_types: &["text/x-teratermmacro"],
        signatures: &[],
        related_formats: &[],
    },
};
