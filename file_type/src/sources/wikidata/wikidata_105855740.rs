use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855740: FileFormat = FileFormat {
    id: 105_855_740,
    puid: "wikidata/105855740",
    name: "Digital Mugician module",
    extensions: &["dmu"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x20, 0x4D, 0x55, 0x47, 0x49, 0x43, 0x49, 0x41, 0x4E, 0x2F, 0x53, 0x4F, 0x46,
                    0x54, 0x45, 0x59, 0x45, 0x53, 0x20, 0x31, 0x39, 0x39, 0x30, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
