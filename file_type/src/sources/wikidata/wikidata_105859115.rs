use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859115: FileFormat = FileFormat {
    id: 105_859_115,
    source_type: SourceType::Wikidata,
    name: "Taquart Interlace Picture bitmap",
    extensions: &["tip"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x49, 0x50, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
