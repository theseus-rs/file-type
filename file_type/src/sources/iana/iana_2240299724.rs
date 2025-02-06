use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2240299724: FileFormat = FileFormat {
    id: 2_240_299_724,
    source_type: SourceType::Iana,
    name: "vnd.cns.anp1",
    extensions: &[],
    media_types: &["audio/vnd.cns.anp1"],
    signatures: &[],
    related_formats: &[],
};
