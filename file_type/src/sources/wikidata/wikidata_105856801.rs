use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856801: FileFormat = FileFormat {
    id: 105_856_801,
    puid: "wikidata/105856801",
    name: "Gosset Graphics object",
    extensions: &["gossett"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x70, 0x6F, 0x6C, 0x79, 0x20, 0x30, 0x0A, 0x20, 0x20, 0x76, 0x65, 0x72, 0x74,
                    0x20, 0x31, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x6C, 0x6F, 0x63,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
