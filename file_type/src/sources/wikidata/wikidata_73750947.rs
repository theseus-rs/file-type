use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73750947: FileFormat = FileFormat {
    id: 73_750_947,
    puid: "wikidata/73750947",
    name: "Q-emulator Configuration",
    extensions: &["qcf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x61, 0x6D, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
