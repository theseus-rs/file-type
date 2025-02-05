use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1424987: FileFormat = FileFormat {
    id: 1_424_987,
    source_type: SourceType::Wikidata,
    name: "Notation3",
    extensions: &["n3"],
    media_types: &["text/n3"],
    signatures: &[],
    related_formats: &[],
};
