use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852525: FileFormat = FileFormat {
    id: 105_852_525,
    source_type: SourceType::Wikidata,
    name: "AKVIS Strokes",
    extensions: &["strokes"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4B, 0x56, 0x49, 0x53, 0x2E, 0x53, 0x54, 0x52, 0x4F, 0x4B, 0x45, 0x53,
                    0x2E, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x2E, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
