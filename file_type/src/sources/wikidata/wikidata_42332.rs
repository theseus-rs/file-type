use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_42332: FileFormat = FileFormat {
    id: 42_332,
    puid: "wikidata/42332",
    name: "PDF",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x50, 0x44, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
