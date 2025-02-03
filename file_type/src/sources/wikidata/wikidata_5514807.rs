use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_5514807: FileFormat = FileFormat {
    id: 5_514_807,
    source_type: SourceType::Wikidata,
    name: "GUIDO music notation",
    extensions: &["gmn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
