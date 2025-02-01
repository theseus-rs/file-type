use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207478: FileFormat = FileFormat {
    id: 28_207_478,
    puid: "wikidata/28207478",
    name: "Webshots picture WBC",
    extensions: &["wbc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAB, 0x16, 0xFA, 0x95])],
            },
        }],
    }],
    related_formats: &[],
};
