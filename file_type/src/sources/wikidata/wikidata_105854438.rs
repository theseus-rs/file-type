use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854438: FileFormat = FileFormat {
    id: 105_854_438,
    puid: "wikidata/105854438",
    name: "Sample DUMP Exchange audio",
    extensions: &["sdx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x44, 0x58, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
