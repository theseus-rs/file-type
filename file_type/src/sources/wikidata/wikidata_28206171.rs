use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206171: FileFormat = FileFormat {
    id: 28_206_171,
    source_type: SourceType::Wikidata,
    name: "GIMP Animated Brush",
    extensions: &["gih"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
