use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_3008299: FileType = FileType {
    file_format: &FileFormat {
        id: 3_008_299,
        source_type: SourceType::Wikidata,
        name: "xorg.conf",
        extensions: &["xorg.conf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
