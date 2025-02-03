use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3400961159: FileFormat = FileFormat {
    id: 3_400_961_159,
    source_type: SourceType::Iana,
    name: "vnd.openblox.game-binary",
    extensions: &[],
    media_types: &["application/vnd.openblox.game-binary"],
    internal_signatures: &[],
    related_formats: &[],
};
