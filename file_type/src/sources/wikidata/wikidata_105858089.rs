use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858089: FileFormat = FileFormat {
    id: 105_858_089,
    source_type: SourceType::Wikidata,
    name: "Adobe Type Manager Font Information",
    extensions: &["inf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x6F, 0x6E, 0x74, 0x4E, 0x61, 0x6D, 0x65, 0x20, 0x28,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
