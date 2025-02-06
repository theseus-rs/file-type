use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859238: FileFormat = FileFormat {
    id: 105_859_238,
    source_type: SourceType::Wikidata,
    name: "AmigaBASIC source (protected)",
    extensions: &["bas"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF4, 0xC2])],
            },
        }],
    }],
    related_formats: &[],
};
