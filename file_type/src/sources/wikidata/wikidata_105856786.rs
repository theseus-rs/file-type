use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856786: FileFormat = FileFormat {
    id: 105_856_786,
    puid: "wikidata/105856786",
    name: "GDLib Image",
    extensions: &["gd2"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x67, 0x64, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
