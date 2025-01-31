use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856327: FileFormat = FileFormat {
    id: 105_856_327,
    puid: "wikidata/105856327",
    name: "RomCenter format",
    extensions: &["dat"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x43, 0x52, 0x45, 0x44, 0x49, 0x54, 0x53, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
