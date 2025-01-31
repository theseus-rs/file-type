use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979394: FileFormat = FileFormat {
    id: 27_979_394,
    puid: "wikidata/27979394",
    name: "DVM",
    extensions: &["dvm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x56, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
