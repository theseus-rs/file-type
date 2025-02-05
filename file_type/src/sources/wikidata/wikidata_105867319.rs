use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867319: FileFormat = FileFormat {
    id: 105_867_319,
    source_type: SourceType::Wikidata,
    name: "Olitext Notes",
    extensions: &["ntp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x54, 0x58, 0x4E, 0x54, 0x50, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
