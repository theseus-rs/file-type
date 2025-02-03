use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_22097440: FileFormat = FileFormat {
    id: 22_097_440,
    source_type: SourceType::Wikidata,
    name: "IPSW",
    extensions: &["ipsw"],
    media_types: &["application/x-itunes-ipsw"],
    internal_signatures: &[],
    related_formats: &[],
};
