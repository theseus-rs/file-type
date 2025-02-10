use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861062: FileFormat = FileFormat {
    id: 105_861_062,
    source_type: SourceType::Wikidata,
    name: "Abuse Level",
    extensions: &["lvl", "spe"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x50, 0x45, 0x43, 0x31, 0x2E, 0x30, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
