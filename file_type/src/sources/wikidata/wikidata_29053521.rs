use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29053521: FileFormat = FileFormat {
    id: 29_053_521,
    puid: "wikidata/29053521",
    name: "Kaitai Struct",
    extensions: &["ksy"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6D, 0x65, 0x74, 0x61, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
