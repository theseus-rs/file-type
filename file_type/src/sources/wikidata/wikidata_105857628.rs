use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857628: FileFormat = FileFormat {
    id: 105_857_628,
    puid: "wikidata/105857628",
    name: "Virtual ][ disk image",
    extensions: &["v2d"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x35, 0x4E, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
