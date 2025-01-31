use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851230: FileFormat = FileFormat {
    id: 105_851_230,
    puid: "wikidata/105851230",
    name: "Text - UTF-EBCDIC encoded",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xDD, 0x73, 0x66, 0x73])],
            },
        }],
    }],
    related_formats: &[],
};
