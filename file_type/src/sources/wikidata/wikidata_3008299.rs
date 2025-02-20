use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
