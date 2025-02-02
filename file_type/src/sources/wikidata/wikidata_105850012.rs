use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850012: FileFormat = FileFormat {
    id: 105_850_012,
    source_type: SourceType::Wikidata,
    name: "WinArcadia Recording/macro",
    extensions: &["cor"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xBF, 0x11, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
