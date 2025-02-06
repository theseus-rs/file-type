use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854926: FileFormat = FileFormat {
    id: 105_854_926,
    source_type: SourceType::Wikidata,
    name: "Art Of Noise MF instrument (v1.x)",
    extensions: &["fm"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4F, 0x4E, 0x2D, 0x46, 0x4D, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
