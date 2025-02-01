use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851980: FileFormat = FileFormat {
    id: 105_851_980,
    puid: "wikidata/105851980",
    name: "Windows Setup Table File",
    extensions: &["stf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x70, 0x70, 0x20, 0x4E, 0x61, 0x6D, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
