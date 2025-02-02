use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850101: FileFormat = FileFormat {
    id: 105_850_101,
    source_type: SourceType::Wikidata,
    name: "ChessBase database Header",
    extensions: &["cbh"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x2C, 0x00, 0x2E, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
