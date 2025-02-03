use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853820: FileFormat = FileFormat {
    id: 105_853_820,
    source_type: SourceType::Wikidata,
    name: "Advanced Installer Updates configuration",
    extensions: &["aiu"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3B, 0x61, 0x69, 0x75, 0x3B, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
