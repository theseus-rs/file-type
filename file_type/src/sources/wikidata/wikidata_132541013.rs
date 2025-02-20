use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132541013: FileType = FileType {
    file_format: &FileFormat {
        id: 132_541_013,
        source_type: SourceType::Wikidata,
        name: "Channel Properties file format",
        extensions: &["cprops"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
