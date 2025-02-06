use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859430: FileFormat = FileFormat {
    id: 105_859_430,
    source_type: SourceType::Wikidata,
    name: "Windows 98 MSBackup backup set",
    extensions: &["qic"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x54, 0x42, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
