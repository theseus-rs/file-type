use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110239092: FileFormat = FileFormat {
    id: 110_239_092,
    source_type: SourceType::Wikidata,
    name: "Avid Editor Format",
    extensions: &["txt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
