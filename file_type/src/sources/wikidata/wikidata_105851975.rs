use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851975: FileFormat = FileFormat {
    id: 105_851_975,
    puid: "wikidata/105851975",
    name: "Windows Explorer Command Shell file",
    extensions: &["scf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x53, 0x68, 0x65, 0x6C, 0x6C, 0x5D, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
