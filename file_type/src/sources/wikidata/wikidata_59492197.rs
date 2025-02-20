use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59492197: FileType = FileType {
    file_format: &FileFormat {
        id: 59_492_197,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System Catalog (Unix)",
        extensions: &["sas7bcat", "sc7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
