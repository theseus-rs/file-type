use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851807: FileFormat = FileFormat {
    id: 105_851_807,
    source_type: SourceType::Wikidata,
    name: "OPTune Saved recovery info",
    extensions: &["sav"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xAA, 0x55, 0x47, 0x41, 0x5A, 0x45, 0x4C, 0x4C, 0x45, 0x5F, 0x53, 0x41, 0x56,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
