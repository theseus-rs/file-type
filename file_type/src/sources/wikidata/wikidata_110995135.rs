use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110995135: FileType = FileType {
    file_format: &FileFormat {
        id: 110_995_135,
        source_type: SourceType::Wikidata,
        name: "Asymetrix 3D Scene",
        extensions: &["scn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
