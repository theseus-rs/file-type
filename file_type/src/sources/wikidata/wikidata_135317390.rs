use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135317390: FileType = FileType {
    file_format: &FileFormat {
        id: 135_317_390,
        source_type: SourceType::Wikidata,
        name: "JFlex grammar specification file",
        extensions: &["jflex"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
