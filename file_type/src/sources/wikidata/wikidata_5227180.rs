use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5227180: FileType = FileType {
    file_format: &FileFormat {
        id: 5_227_180,
        source_type: SourceType::Wikidata,
        name: "Data Interchange Format",
        extensions: &["dif"],
        media_types: &["application/x-dif"],
        signatures: &[],
        related_formats: &[],
    },
};
