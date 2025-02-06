use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858763: FileFormat = FileFormat {
    id: 105_858_763,
    source_type: SourceType::Wikidata,
    name: "Siegfried Antivirus Professional virus data (v1.x)",
    extensions: &["brainfile"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x42, 0x46, 0x56, 0x31, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
