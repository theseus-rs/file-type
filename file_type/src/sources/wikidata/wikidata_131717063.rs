use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131717063: FileType = FileType {
    file_format: &FileFormat {
        id: 131_717_063,
        source_type: SourceType::Wikidata,
        name: "AVS UCD Binary/ASCII file format",
        extensions: &["inp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
