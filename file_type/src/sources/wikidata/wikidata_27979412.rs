use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979412: FileFormat = FileFormat {
    id: 27_979_412,
    puid: "wikidata/27979412",
    name: "RIPscrip",
    extensions: &["rip"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x21, 0x7C])],
            },
        }],
    }],
    related_formats: &[],
};
