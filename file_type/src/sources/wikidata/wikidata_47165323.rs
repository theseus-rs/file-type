use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47165323: FileType = FileType {
    file_format: &FileFormat {
        id: 47_165_323,
        source_type: SourceType::Wikidata,
        name: "Microsoft Project file format, version 2000-2003",
        extensions: &["mpp"],
        media_types: &["application/vnd.ms-project"],
        signatures: &[],
        related_formats: &[],
    },
};
