use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131304603: FileType = FileType {
    file_format: &FileFormat {
        id: 131_304_603,
        source_type: SourceType::Wikidata,
        name: "Transact-SQL file format",
        extensions: &["sql"],
        media_types: &["text/x-tsql"],
        signatures: &[],
        related_formats: &[],
    },
};
