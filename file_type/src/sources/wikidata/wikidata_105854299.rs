use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854299: FileFormat = FileFormat {
    id: 105_854_299,
    source_type: SourceType::Wikidata,
    name: "Advanced Data Format data base",
    extensions: &["adf"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x40, 0x28, 0x23, 0x29, 0x41, 0x44, 0x46, 0x20, 0x44, 0x61, 0x74, 0x61, 0x62,
                    0x61, 0x73, 0x65, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x41,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
