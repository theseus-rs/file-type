use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3352933827: FileFormat = FileFormat {
    id: 3_352_933_827,
    source_type: SourceType::Iana,
    name: "vnd.llamagraphics.life-balance.desktop",
    extensions: &[],
    media_types: &["application/vnd.llamagraphics.life-balance.desktop"],
    internal_signatures: &[],
    related_formats: &[],
};
