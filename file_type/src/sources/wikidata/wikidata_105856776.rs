use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856776: FileFormat = FileFormat {
    id: 105_856_776,
    puid: "wikidata/105856776",
    name: "MikMod module",
    extensions: &["uni"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x4E, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
