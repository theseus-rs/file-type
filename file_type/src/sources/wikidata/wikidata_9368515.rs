use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_9368515: FileFormat = FileFormat {
    id: 9_368_515,
    source_type: SourceType::Wikidata,
    name: "MFS",
    extensions: &["mfs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
