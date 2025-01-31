use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851222: FileFormat = FileFormat {
    id: 105_851_222,
    puid: "wikidata/105851222",
    name: "Harvard Graphics Template (v2.x)",
    extensions: &["tpl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x74, 0x70, 0x6C, 0x74, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
