use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_58103077: FileType = FileType {
    file_format: &FileFormat {
        id: 58_103_077,
        source_type: SourceType::Wikidata,
        name: "LifeTechnologies SDS",
        extensions: &["sds"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
