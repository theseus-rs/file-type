use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979397: FileFormat = FileFormat {
    id: 27_979_397,
    puid: "wikidata/27979397",
    name: "Imagic Film/Picture, low resolution",
    extensions: &["ic1"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x4D, 0x44, 0x43, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
