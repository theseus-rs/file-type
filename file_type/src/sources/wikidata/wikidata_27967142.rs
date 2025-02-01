use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967142: FileFormat = FileFormat {
    id: 27_967_142,
    puid: "wikidata/27967142",
    name: "Digitrakker sample",
    extensions: &["spl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x53, 0x50, 0x4C, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
