use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133146964: FileType = FileType {
    file_format: &FileFormat {
        id: 133_146_964,
        source_type: SourceType::Wikidata,
        name: "Microsoft Project file, version 1",
        extensions: &["mpp"],
        media_types: &["application/vnd.ms-project"],
        signatures: &[],
        related_formats: &[],
    },
};
