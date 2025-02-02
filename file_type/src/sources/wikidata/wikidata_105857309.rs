use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857309: FileFormat = FileFormat {
    id: 105_857_309,
    source_type: SourceType::Wikidata,
    name: "Hexels drawing",
    extensions: &["hxl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x45, 0x58, 0x45, 0x4C, 0x53, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
