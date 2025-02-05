use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864196: FileFormat = FileFormat {
    id: 105_864_196,
    source_type: SourceType::Wikidata,
    name: "Test Drive PC shapes game data",
    extensions: &["pes"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x63, 0x6B, 0x64])],
            },
        }],
    }],
    related_formats: &[],
};
