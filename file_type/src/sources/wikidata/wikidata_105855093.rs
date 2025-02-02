use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855093: FileFormat = FileFormat {
    id: 105_855_093,
    source_type: SourceType::Wikidata,
    name: "DLC - DIGILINEAR compressed archive",
    extensions: &["dlc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x4C, 0x43, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
