use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130372599: FileFormat = FileFormat {
    id: 130_372_599,
    source_type: SourceType::Wikidata,
    name: "NestedText file format",
    extensions: &["nt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
