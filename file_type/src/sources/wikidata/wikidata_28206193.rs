use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206193: FileFormat = FileFormat {
    id: 28_206_193,
    source_type: SourceType::Wikidata,
    name: "GoDot 4-bit image",
    extensions: &["4bt"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x4F, 0x44, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
