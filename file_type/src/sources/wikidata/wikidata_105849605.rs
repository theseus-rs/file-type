use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849605: FileFormat = FileFormat {
    id: 105_849_605,
    source_type: SourceType::Wikidata,
    name: "ChessBase Light database Header",
    extensions: &["cbh"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x24, 0x00, 0x2E, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
