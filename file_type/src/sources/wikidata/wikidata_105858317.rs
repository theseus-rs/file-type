use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858317: FileFormat = FileFormat {
    id: 105_858_317,
    puid: "wikidata/105858317",
    name: "Melco embroidery design",
    extensions: &["exp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x80, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
