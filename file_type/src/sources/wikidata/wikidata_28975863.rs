use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975863: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_863,
        source_type: SourceType::Wikidata,
        name: "OOGL Bezier Surface BEZ",
        extensions: &["bez"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
