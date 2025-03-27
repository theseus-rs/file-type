use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113866096: FileType = FileType {
    file_format: &FileFormat {
        id: 113_866_096,
        source_type: SourceType::Wikidata,
        name: "Apple Partition Map Disk Image",
        extensions: &["bin", "cdr", "dmg", "img", "iso", "toast"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
