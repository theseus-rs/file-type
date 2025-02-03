use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852975: FileFormat = FileFormat {
    id: 105_852_975,
    source_type: SourceType::Wikidata,
    name: "SBF game data container",
    extensions: &["sbf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x42, 0x46, 0x30, 0x00, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
