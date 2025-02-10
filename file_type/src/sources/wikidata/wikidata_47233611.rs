use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47233611: FileType = FileType {
    file_format: &FileFormat {
        id: 47_233_611,
        source_type: SourceType::Wikidata,
        name: "MPD",
        extensions: &["mpd"],
        media_types: &["application/x-multi-part-ldraw"],
        signatures: &[],
        related_formats: &[],
    },
};
