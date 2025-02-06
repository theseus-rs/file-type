use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855284: FileFormat = FileFormat {
    id: 105_855_284,
    source_type: SourceType::Wikidata,
    name: "FCEUX movie capture",
    extensions: &["fm2"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x33, 0x0A, 0x65, 0x6D, 0x75,
                    0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
