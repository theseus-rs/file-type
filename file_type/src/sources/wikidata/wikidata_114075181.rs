use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114075181: FileType = FileType {
    file_format: &FileFormat {
        id: 114_075_181,
        source_type: SourceType::Wikidata,
        name: "Apple Partition Map - ISO 9660 - UDF Hybrid Disk Image",
        extensions: &["dmg", "iso", "toast"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
