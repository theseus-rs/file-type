use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125077026: FileFormat = FileFormat {
    id: 125_077_026,
    source_type: SourceType::Wikidata,
    name: "Gregorian chant score file",
    extensions: &["gabc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
