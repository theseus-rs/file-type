use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854189: FileFormat = FileFormat {
    id: 105_854_189,
    puid: "wikidata/105854189",
    name: "SEMONE compressed archive",
    extensions: &["one"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x45, 0x4D, 0x68, 0x29])],
            },
        }],
    }],
    related_formats: &[],
};
