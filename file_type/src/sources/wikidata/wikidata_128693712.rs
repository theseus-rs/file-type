use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128693712: FileType = FileType {
    file_format: &FileFormat {
        id: 128_693_712,
        source_type: SourceType::Wikidata,
        name: "Berry source code file",
        extensions: &["be"],
        media_types: &["application/x-berry", "text/x-berry"],
        signatures: &[],
        related_formats: &[],
    },
};
