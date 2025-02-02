use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119519667: FileFormat = FileFormat {
    id: 119_519_667,
    source_type: SourceType::Wikidata,
    name: "DubIt Project",
    extensions: &["dub"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
