use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861102: FileFormat = FileFormat {
    id: 105_861_102,
    source_type: SourceType::Wikidata,
    name: "Cabrillo Log (v2.0)",
    extensions: &["log"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x54, 0x41, 0x52, 0x54, 0x2D, 0x4F, 0x46, 0x2D, 0x4C, 0x4F, 0x47, 0x3A,
                    0x20, 0x32, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
