use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861793: FileFormat = FileFormat {
    id: 105_861_793,
    source_type: SourceType::Wikidata,
    name: "Maple Common Binary (Amiga)",
    extensions: &["m"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x04, 0x32, 0x96, 0x41, 0x00, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
