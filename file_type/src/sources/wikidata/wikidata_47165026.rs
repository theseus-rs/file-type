use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47165026: FileType = FileType {
    file_format: &FileFormat {
        id: 47_165_026,
        source_type: SourceType::Wikidata,
        name: "Microsoft Project file format, version 95",
        extensions: &["mpp"],
        media_types: &["application/vnd.ms-project"],
        signatures: &[],
        related_formats: &[],
    },
};
