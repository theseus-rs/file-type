use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87119731: FileFormat = FileFormat {
    id: 87_119_731,
    source_type: SourceType::Wikidata,
    name: "Open Financial Exchange 1.02",
    extensions: &["ofx", "qfx"],
    media_types: &["application/x-ofx"],
    internal_signatures: &[],
    related_formats: &[],
};
