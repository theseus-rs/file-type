use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131303765: FileType = FileType {
    file_format: &FileFormat {
        id: 131_303_765,
        source_type: SourceType::Wikidata,
        name: "TL-b source code file",
        extensions: &["tlb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
