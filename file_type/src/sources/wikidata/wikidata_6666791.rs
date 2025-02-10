use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_6666791: FileType = FileType {
    file_format: &FileFormat {
        id: 6_666_791,
        source_type: SourceType::Wikidata,
        name: "Log ASCII Standard Format",
        extensions: &["las"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
