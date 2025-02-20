use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_722609: FileType = FileType {
    file_format: &FileFormat {
        id: 722_609,
        source_type: SourceType::Wikidata,
        name: "MARC standards",
        extensions: &["marc", "mrc"],
        media_types: &["application/marc"],
        signatures: &[],
        related_formats: &[],
    },
};
