use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855430: FileFormat = FileFormat {
    id: 105_855_430,
    puid: "wikidata/105855430",
    name: "Organize! Form",
    extensions: &["frm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xCD, 0xCA])],
            },
        }],
    }],
    related_formats: &[],
};
