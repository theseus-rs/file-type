use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967224: FileFormat = FileFormat {
    id: 27_967_224,
    puid: "wikidata/27967224",
    name: "Ultra Tracker",
    extensions: &["ult"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x41, 0x53, 0x5F, 0x55, 0x54, 0x72, 0x61, 0x63, 0x6B, 0x5F, 0x56,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
