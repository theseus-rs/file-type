use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858854: FileFormat = FileFormat {
    id: 105_858_854,
    puid: "wikidata/105858854",
    name: "Pascal Script binary",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x46, 0x50, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
