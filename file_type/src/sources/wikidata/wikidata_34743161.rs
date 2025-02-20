use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34743161: FileType = FileType {
    file_format: &FileFormat {
        id: 34_743_161,
        source_type: SourceType::Wikidata,
        name: "Softdisk Publishing UDF",
        extensions: &["udf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
