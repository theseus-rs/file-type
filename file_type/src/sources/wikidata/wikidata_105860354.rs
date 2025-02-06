use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860354: FileFormat = FileFormat {
    id: 105_860_354,
    source_type: SourceType::Wikidata,
    name: "Borland Turbo Vision Resource",
    extensions: &["res"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x42, 0x50, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
