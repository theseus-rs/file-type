use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112218888: FileFormat = FileFormat {
    id: 112_218_888,
    puid: "wikidata/112218888",
    name: "Adobe Audition Peak File",
    extensions: &["pkf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x69, 0x24, 0x21, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
