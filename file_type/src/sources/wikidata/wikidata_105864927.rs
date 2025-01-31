use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864927: FileFormat = FileFormat {
    id: 105_864_927,
    puid: "wikidata/105864927",
    name: "BIS Visitor Project",
    extensions: &["pew"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4F, 0x53, 0x45, 0x57])],
            },
        }],
    }],
    related_formats: &[],
};
