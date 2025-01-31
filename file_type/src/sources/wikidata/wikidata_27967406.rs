use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967406: FileFormat = FileFormat {
    id: 27_967_406,
    puid: "wikidata/27967406",
    name: "Reality AdLib Tracker module",
    extensions: &["rad"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x41, 0x44, 0x20, 0x62, 0x79, 0x20, 0x52, 0x45, 0x41, 0x4C, 0x69, 0x54,
                    0x59, 0x21, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
