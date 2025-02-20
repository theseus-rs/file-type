use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47455968: FileType = FileType {
    file_format: &FileFormat {
        id: 47_455_968,
        source_type: SourceType::Wikidata,
        name: "SafeGuard Encrypted Virtual Disk",
        extensions: &["hdr", "vol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
