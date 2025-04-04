use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975633: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_633,
        source_type: SourceType::Wikidata,
        name: "NextEngine Scan",
        extensions: &["scn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
