use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861068: FileFormat = FileFormat {
    id: 105_861_068,
    source_type: SourceType::Wikidata,
    name: "Look and Feel screen",
    extensions: &["lnf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x4E, 0x46, 0x56, 0x45, 0x52, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
