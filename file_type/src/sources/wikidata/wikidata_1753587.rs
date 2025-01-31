use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1753587: FileFormat = FileFormat {
    id: 1_753_587,
    puid: "wikidata/1753587",
    name: "WBFS",
    extensions: &["wbfs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x42, 0x46, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
