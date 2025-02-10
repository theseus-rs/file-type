use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2609791: FileType = FileType {
    file_format: &FileFormat {
        id: 2_609_791,
        source_type: SourceType::Wikidata,
        name: "Blu-ray Disc Audio-Video MPEG-2 Transport Stream container file format",
        extensions: &["MTS", "m2ts"],
        media_types: &["video/MP2T"],
        signatures: &[],
        related_formats: &[],
    },
};
