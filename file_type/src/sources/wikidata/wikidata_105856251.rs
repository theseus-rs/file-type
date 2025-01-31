use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856251: FileFormat = FileFormat {
    id: 105_856_251,
    puid: "wikidata/105856251",
    name: "Dynamic Publisher Document",
    extensions: &["doc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x79, 0x6E, 0x61, 0x6D, 0x69, 0x63, 0x20, 0x50, 0x75, 0x62, 0x6C, 0x69,
                    0x73, 0x68, 0x65, 0x72, 0x20, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
