use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856882: FileFormat = FileFormat {
    id: 105_856_882,
    puid: "wikidata/105856882",
    name: "GlueMon module",
    extensions: &["glue"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
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
