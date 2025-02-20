use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128694654: FileType = FileType {
    file_format: &FileFormat {
        id: 128_694_654,
        source_type: SourceType::Wikidata,
        name: "Carbon file format",
        extensions: &["carbon"],
        media_types: &["text/x-carbon"],
        signatures: &[],
        related_formats: &[],
    },
};
