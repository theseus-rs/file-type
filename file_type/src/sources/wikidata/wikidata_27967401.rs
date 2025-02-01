use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967401: FileFormat = FileFormat {
    id: 27_967_401,
    puid: "wikidata/27967401",
    name: "XMS-Tracker module",
    extensions: &["xms"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x61, 0x44, 0x6F, 0x4B, 0x61, 0x4E, 0x39, 0x36,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
