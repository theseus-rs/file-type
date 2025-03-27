use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205434: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_434,
        source_type: SourceType::Wikidata,
        name: "Samsung SRW",
        extensions: &["srw"],
        media_types: &["image/x-samsung-srw"],
        signatures: &[],
        related_formats: &[],
    },
};
