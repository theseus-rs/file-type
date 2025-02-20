use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128596448: FileType = FileType {
    file_format: &FileFormat {
        id: 128_596_448,
        source_type: SourceType::Wikidata,
        name: "Alloy file format",
        extensions: &["als"],
        media_types: &["text/x-alloy"],
        signatures: &[],
        related_formats: &[],
    },
};
