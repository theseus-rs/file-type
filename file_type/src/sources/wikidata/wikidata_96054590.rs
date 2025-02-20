use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_96054590: FileType = FileType {
    file_format: &FileFormat {
        id: 96_054_590,
        source_type: SourceType::Wikidata,
        name: "Macromolecular Crystallographic Information File",
        extensions: &["mmcif"],
        media_types: &["chemical/x-mmcif"],
        signatures: &[],
        related_formats: &[],
    },
};
