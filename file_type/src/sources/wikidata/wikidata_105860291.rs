use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860291: FileFormat = FileFormat {
    id: 105_860_291,
    source_type: SourceType::Wikidata,
    name: "M.A.X. game data archive",
    extensions: &["res"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x45, 0x53, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
