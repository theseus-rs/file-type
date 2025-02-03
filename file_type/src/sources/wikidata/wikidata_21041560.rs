use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_21041560: FileFormat = FileFormat {
    id: 21_041_560,
    source_type: SourceType::Wikidata,
    name: "Oktalyzer format",
    extensions: &["okt", "okta"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
