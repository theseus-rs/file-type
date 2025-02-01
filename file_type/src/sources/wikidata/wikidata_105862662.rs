use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862662: FileFormat = FileFormat {
    id: 105_862_662,
    puid: "wikidata/105862662",
    name: "OGRE Material",
    extensions: &["material"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6D, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61, 0x6C, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
