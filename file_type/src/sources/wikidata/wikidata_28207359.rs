use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207359: FileFormat = FileFormat {
    id: 28_207_359,
    puid: "wikidata/28207359",
    name: "TAP",
    extensions: &["tap"],
    media_types: &["image/vnd.tencent.tap"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x41, 0x50, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
