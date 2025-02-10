use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110015870: FileType = FileType {
    file_format: &FileFormat {
        id: 110_015_870,
        source_type: SourceType::Wikidata,
        name: "InstallShield Executable",
        extensions: &["ex_"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
