use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979224: FileFormat = FileFormat {
    id: 27_979_224,
    source_type: SourceType::Wikidata,
    name: "Advanced Video Attribute Terminal Assembler and Recreator",
    extensions: &["avt", "bbs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
