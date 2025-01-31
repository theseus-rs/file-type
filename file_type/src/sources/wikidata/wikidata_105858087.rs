use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858087: FileFormat = FileFormat {
    id: 105_858_087,
    puid: "wikidata/105858087",
    name: "NorthStar disk Image (NS DOS)",
    extensions: &["nsi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x4F, 0x53, 0x20, 0x20, 0x20, 0x20, 0x20, 0x04, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
