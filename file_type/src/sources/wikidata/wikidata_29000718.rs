use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000718: FileFormat = FileFormat {
    id: 29_000_718,
    source_type: SourceType::Wikidata,
    name: "Unreal model aniv file",
    extensions: &["3d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
