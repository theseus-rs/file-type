use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2266263228: FileType = FileType {
    file_format: &FileFormat {
        id: 2_266_263_228,
        source_type: SourceType::Iana,
        name: "TETRA_ACELP_BB",
        extensions: &[],
        media_types: &["audio/TETRA_ACELP_BB"],
        signatures: &[],
        related_formats: &[],
    },
};
