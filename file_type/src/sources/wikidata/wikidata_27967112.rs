use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967112: FileFormat = FileFormat {
    id: 27_967_112,
    puid: "wikidata/27967112",
    name: "All Sound Tracker module",
    extensions: &["ast"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x08, 0x41, 0x53, 0x54, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
