use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858740: FileFormat = FileFormat {
    id: 105_858_740,
    puid: "wikidata/105858740",
    name: "packPNM compressed BMP bitmap",
    extensions: &["ppn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
