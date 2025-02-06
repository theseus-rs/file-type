use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861193: FileFormat = FileFormat {
    id: 105_861_193,
    source_type: SourceType::Wikidata,
    name: "Levelogger Software Data",
    extensions: &["lev"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x61, 0x74, 0x61, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x66, 0x6F, 0x72,
                    0x20, 0x44, 0x61, 0x74, 0x61, 0x4C, 0x6F, 0x67, 0x67, 0x65, 0x72, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
