use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850377: FileFormat = FileFormat {
    id: 105_850_377,
    source_type: SourceType::Wikidata,
    name: "ChessBase Archive file",
    extensions: &["cbv"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x08, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
