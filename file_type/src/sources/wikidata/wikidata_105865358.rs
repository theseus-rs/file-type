use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865358: FileFormat = FileFormat {
    id: 105_865_358,
    puid: "wikidata/105865358",
    name: "MicroImages Package",
    extensions: &["pkg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x49, 0x20, 0x50, 0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x20, 0x46, 0x69,
                    0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
