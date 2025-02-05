use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857504: FileFormat = FileFormat {
    id: 105_857_504,
    source_type: SourceType::Wikidata,
    name: "3Doku game",
    extensions: &["3doku"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x33, 0x44, 0x6F, 0x6B, 0x75, 0x47, 0x61, 0x6D, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
