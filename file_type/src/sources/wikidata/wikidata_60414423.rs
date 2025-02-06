use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60414423: FileFormat = FileFormat {
    id: 60_414_423,
    source_type: SourceType::Wikidata,
    name: "TAP (Commodore 64)",
    extensions: &["tap"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
