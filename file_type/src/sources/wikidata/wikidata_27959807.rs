use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27959807: FileFormat = FileFormat {
    id: 27_959_807,
    source_type: SourceType::Wikidata,
    name: "Ableton Live Pack",
    extensions: &["alp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
