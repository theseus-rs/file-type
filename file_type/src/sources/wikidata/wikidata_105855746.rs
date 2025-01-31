use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855746: FileFormat = FileFormat {
    id: 105_855_746,
    puid: "wikidata/105855746",
    name: "InterActual Disc.id",
    extensions: &["id"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x49, 0x6E, 0x74, 0x65, 0x72, 0x41, 0x63, 0x74, 0x75, 0x61, 0x6C, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
