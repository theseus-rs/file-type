use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979390: FileFormat = FileFormat {
    id: 27_979_390,
    puid: "wikidata/27979390",
    name: "Animatic Film",
    extensions: &["flm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x27, 0x18, 0x28, 0x18])],
            },
        }],
    }],
    related_formats: &[],
};
