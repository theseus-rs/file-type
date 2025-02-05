use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1315657: FileFormat = FileFormat {
    id: 1_315_657,
    source_type: SourceType::Wikidata,
    name: "Textile",
    extensions: &["textile"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
