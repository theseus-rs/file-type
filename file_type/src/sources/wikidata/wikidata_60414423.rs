use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60414423: FileFormat = FileFormat {
    id: 60_414_423,
    source_type: SourceType::Wikidata,
    name: "TAP (Commodore 64)",
    extensions: &["tap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
