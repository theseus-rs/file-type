use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979224: FileFormat = FileFormat {
    id: 27_979_224,
    source_type: SourceType::Wikidata,
    name: "Advanced Video Attribute Terminal Assembler and Recreator",
    extensions: &["avt", "bbs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
