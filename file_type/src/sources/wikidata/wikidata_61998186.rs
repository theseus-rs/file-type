use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61998186: FileFormat = FileFormat {
    id: 61_998_186,
    source_type: SourceType::Wikidata,
    name: "Feather file format",
    extensions: &["feather"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0xA1])],
            },
        }],
    }],
    related_formats: &[],
};
