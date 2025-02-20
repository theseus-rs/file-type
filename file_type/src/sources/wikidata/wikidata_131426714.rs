use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131426714: FileType = FileType {
    file_format: &FileFormat {
        id: 131_426_714,
        source_type: SourceType::Wikidata,
        name: "X++ source code file format",
        extensions: &["xpp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
