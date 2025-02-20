use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61913345: FileType = FileType {
    file_format: &FileFormat {
        id: 61_913_345,
        source_type: SourceType::Wikidata,
        name: "Microsoft Project Export File, version 3",
        extensions: &["mpx"],
        media_types: &["application/x-project"],
        signatures: &[],
        related_formats: &[],
    },
};
