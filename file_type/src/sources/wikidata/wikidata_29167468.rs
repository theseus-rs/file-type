use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29167468: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_468,
        source_type: SourceType::Wikidata,
        name: "Open Virtualization Format Archive Package",
        extensions: &["ova"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
