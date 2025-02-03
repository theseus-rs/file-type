use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858984: FileFormat = FileFormat {
    id: 105_858_984,
    source_type: SourceType::Wikidata,
    name: "Wwise sound Bank",
    extensions: &["bnk"],
    media_types: &["application/vnd.wwise.bnk"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x4B, 0x48, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
