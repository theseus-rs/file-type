use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861030: FileFormat = FileFormat {
    id: 105_861_030,
    source_type: SourceType::Wikidata,
    name: "Passolo Localization Project",
    extensions: &["lpu"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x51, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
