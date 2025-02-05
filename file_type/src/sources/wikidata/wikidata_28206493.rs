use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206493: FileFormat = FileFormat {
    id: 28_206_493,
    source_type: SourceType::Wikidata,
    name: "Lightning Strike",
    extensions: &["cod"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
