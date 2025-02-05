use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864796: FileFormat = FileFormat {
    id: 105_864_796,
    source_type: SourceType::Wikidata,
    name: "Infernal Engine game data archive",
    extensions: &["pod"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4F, 0x44, 0x35])],
            },
        }],
    }],
    related_formats: &[],
};
