use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959807: FileFormat = FileFormat {
    id: 27_959_807,
    source_type: SourceType::Wikidata,
    name: "Ableton Live Pack",
    extensions: &["alp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
