use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34745668: FileFormat = FileFormat {
    id: 34_745_668,
    source_type: SourceType::Wikidata,
    name: "Squeeze",
    extensions: &["qqq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
