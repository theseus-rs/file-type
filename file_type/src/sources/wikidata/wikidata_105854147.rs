use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854147: FileFormat = FileFormat {
    id: 105_854_147,
    puid: "wikidata/105854147",
    name: "LzTurbo compressed",
    extensions: &["lzt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x4C, 0x5A, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
