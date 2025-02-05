use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856882: FileFormat = FileFormat {
    id: 105_856_882,
    source_type: SourceType::Wikidata,
    name: "GlueMon module",
    extensions: &["glue"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x4C, 0x55, 0x45, 0xB8, 0xB3, 0xAA, 0xBA,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
