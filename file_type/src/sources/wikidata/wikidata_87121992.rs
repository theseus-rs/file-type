use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87121992: FileFormat = FileFormat {
    id: 87_121_992,
    source_type: SourceType::Wikidata,
    name: "Open Financial Exchange 2.0.3",
    extensions: &["ofx", "qfx"],
    media_types: &["application/x-ofx"],
    internal_signatures: &[],
    related_formats: &[],
};
