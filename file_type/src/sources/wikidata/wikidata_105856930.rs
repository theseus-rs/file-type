use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856930: FileFormat = FileFormat {
    id: 105_856_930,
    puid: "wikidata/105856930",
    name: "Amigaguide hypertext document",
    extensions: &["guide"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x40, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
