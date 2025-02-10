use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111272276: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_276,
        source_type: SourceType::Wikidata,
        name: "Ensoniq Mirage disk image file",
        extensions: &["edm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
