use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855822: FileFormat = FileFormat {
    id: 105_855_822,
    puid: "wikidata/105855822",
    name: "Deep Paint 3D Project",
    extensions: &["dp3"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x65, 0x65, 0x70, 0x20, 0x50, 0x61, 0x69, 0x6E, 0x74, 0x20, 0x33, 0x44,
                    0x20, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
