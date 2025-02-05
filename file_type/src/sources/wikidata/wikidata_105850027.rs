use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850027: FileFormat = FileFormat {
    id: 105_850_027,
    source_type: SourceType::Wikidata,
    name: "Calamus Vector Document",
    extensions: &["cvd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x4D, 0x43, 0x20, 0x43, 0x41, 0x4C, 0x41, 0x4D, 0x55, 0x53, 0x20, 0x20,
                    0x43, 0x56, 0x44,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
