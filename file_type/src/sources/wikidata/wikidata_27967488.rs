use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967488: FileFormat = FileFormat {
    id: 27_967_488,
    source_type: SourceType::Wikidata,
    name: "FLV",
    extensions: &["flv"],
    media_types: &["video/x-flv"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4C, 0x56, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
