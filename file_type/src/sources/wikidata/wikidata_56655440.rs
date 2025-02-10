use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_56655440: FileType = FileType {
    file_format: &FileFormat {
        id: 56_655_440,
        source_type: SourceType::Wikidata,
        name: "HDT",
        extensions: &["hdt"],
        media_types: &["application/vnd.hdt"],
        signatures: &[],
        related_formats: &[],
    },
};
