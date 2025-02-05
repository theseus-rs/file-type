use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_21041560: FileFormat = FileFormat {
    id: 21_041_560,
    source_type: SourceType::Wikidata,
    name: "Oktalyzer format",
    extensions: &["okt", "okta"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
