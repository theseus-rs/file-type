use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83794070: FileFormat = FileFormat {
    id: 83_794_070,
    source_type: SourceType::Wikidata,
    name: "EclipseCrossword Puzzle File",
    extensions: &["ecw"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3B, 0x20, 0x45, 0x63, 0x6C, 0x69, 0x70, 0x73, 0x65, 0x43, 0x72, 0x6F, 0x73,
                    0x73, 0x77, 0x6F, 0x72, 0x64, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
