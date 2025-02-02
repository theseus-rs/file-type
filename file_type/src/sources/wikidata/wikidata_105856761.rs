use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856761: FileFormat = FileFormat {
    id: 105_856_761,
    source_type: SourceType::Wikidata,
    name: "Unreal Package",
    extensions: &["uasset"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xC1, 0x83, 0x2A, 0x9E])],
            },
        }],
    }],
    related_formats: &[],
};
