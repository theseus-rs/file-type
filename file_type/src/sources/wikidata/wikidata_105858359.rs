use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858359: FileFormat = FileFormat {
    id: 105_858_359,
    source_type: SourceType::Wikidata,
    name: "E-Studio 1.x experiment",
    extensions: &["es"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
