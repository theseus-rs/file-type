use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29904505: FileFormat = FileFormat {
    id: 29_904_505,
    source_type: SourceType::Wikidata,
    name: "S7z",
    extensions: &["s7z"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
