use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114049059: FileFormat = FileFormat {
    id: 114_049_059,
    source_type: SourceType::Wikidata,
    name: "Rocky Interlace Picture",
    extensions: &["rip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
