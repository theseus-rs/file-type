use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_4027909: FileType = FileType {
    file_format: &FileFormat {
        id: 4_027_909,
        source_type: SourceType::Wikidata,
        name: "Network Bootable Image",
        extensions: &["nbi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
