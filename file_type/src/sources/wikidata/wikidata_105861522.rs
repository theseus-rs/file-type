use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861522: FileFormat = FileFormat {
    id: 105_861_522,
    puid: "wikidata/105861522",
    name: "LogTag Data",
    extensions: &["ltd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x6F, 0x67, 0x54, 0x61, 0x67, 0x20, 0x50, 0x72, 0x6F, 0x64, 0x75, 0x63,
                    0x74, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x0A, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
