use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853245: FileFormat = FileFormat {
    id: 105_853_245,
    puid: "wikidata/105853245",
    name: "Squeak package",
    extensions: &["sar"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x22, 0x53, 0x71, 0x75, 0x65, 0x61, 0x6B, 0x20, 0x61, 0x72, 0x63, 0x68, 0x69,
                    0x76, 0x65, 0x20, 0x77, 0x72, 0x69, 0x74, 0x74, 0x65, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
