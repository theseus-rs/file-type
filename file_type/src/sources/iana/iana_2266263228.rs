use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2266263228: FileFormat = FileFormat {
    id: 2_266_263_228,
    source_type: SourceType::Iana,
    name: "TETRA_ACELP_BB",
    extensions: &[],
    media_types: &["audio/TETRA_ACELP_BB"],
    internal_signatures: &[],
    related_formats: &[],
};
