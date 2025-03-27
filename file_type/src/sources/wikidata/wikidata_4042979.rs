use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_4042979: FileType = FileType {
    file_format: &FileFormat {
        id: 4_042_979,
        source_type: SourceType::Wikidata,
        name: "Linear Executable",
        extensions: &["386", "dll", "exe", "sys", "vxd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
