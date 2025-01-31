use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856946: FileFormat = FileFormat {
    id: 105_856_946,
    puid: "wikidata/105856946",
    name: "Quake 3 Arena skeleton data",
    extensions: &["gla"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x32, 0x4C, 0x47, 0x41, 0x06, 0x00, 0x00, 0x00, 0x6D, 0x6F, 0x64, 0x65, 0x6C,
                    0x73, 0x2F, 0x70, 0x6C, 0x61, 0x79, 0x65, 0x72, 0x73, 0x2F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
