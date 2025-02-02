use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855313: FileFormat = FileFormat {
    id: 105_855_313,
    source_type: SourceType::Wikidata,
    name: "Fluke View data",
    extensions: &["fvf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x56, 0x2E, 0x46, 0x56, 0x46, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
