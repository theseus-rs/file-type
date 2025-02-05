use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34745668: FileFormat = FileFormat {
    id: 34_745_668,
    source_type: SourceType::Wikidata,
    name: "Squeeze",
    extensions: &["qqq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
