use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858724: FileFormat = FileFormat {
    id: 105_858_724,
    source_type: SourceType::Wikidata,
    name: "AMI BIOS Energy Star logo bitmap",
    extensions: &["gle"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x4C, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
