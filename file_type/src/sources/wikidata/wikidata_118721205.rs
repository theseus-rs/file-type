use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118721205: FileFormat = FileFormat {
    id: 118_721_205,
    source_type: SourceType::Wikidata,
    name: "PZZ File",
    extensions: &["pzz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
