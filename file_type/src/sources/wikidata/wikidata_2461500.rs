use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2461500: FileType = FileType {
    file_format: &FileFormat {
        id: 2_461_500,
        source_type: SourceType::Wikidata,
        name: "iOS application archive",
        extensions: &["ipa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
