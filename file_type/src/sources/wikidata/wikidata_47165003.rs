use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47165003: FileType = FileType {
    file_format: &FileFormat {
        id: 47_165_003,
        source_type: SourceType::Wikidata,
        name: "Microsoft Project file format, version 4",
        extensions: &["mpp"],
        media_types: &["application/vnd.ms-project"],
        signatures: &[],
        related_formats: &[],
    },
};
