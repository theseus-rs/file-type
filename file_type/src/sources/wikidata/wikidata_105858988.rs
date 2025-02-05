use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858988: FileFormat = FileFormat {
    id: 105_858_988,
    source_type: SourceType::Wikidata,
    name: "Binary Revolution Font",
    extensions: &["brfnt"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x46, 0x4E, 0x54, 0xFE, 0xFF])],
            },
        }],
    }],
    related_formats: &[],
};
