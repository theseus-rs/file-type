use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67384103: FileFormat = FileFormat {
    id: 67_384_103,
    source_type: SourceType::Wikidata,
    name: "ArtMoney Table File",
    extensions: &["amt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
