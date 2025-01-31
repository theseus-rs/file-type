use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851972: FileFormat = FileFormat {
    id: 105_851_972,
    puid: "wikidata/105851972",
    name: "Psion Archive Screen",
    extensions: &["scn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x64, 0x62, 0x73, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
