use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856289: FileFormat = FileFormat {
    id: 105_856_289,
    source_type: SourceType::Wikidata,
    name: "Freelance Graphics Drawing",
    extensions: &["drw"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x07, 0x00, 0x67, 0x77, 0x74, 0x72, 0x65, 0x65, 0x00, 0x28,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
