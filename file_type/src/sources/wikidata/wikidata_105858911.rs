use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858911: FileFormat = FileFormat {
    id: 105_858_911,
    puid: "wikidata/105858911",
    name: "raw Group 3 FAX bitmap",
    extensions: &["g3"],
    media_types: &["image/g3fax"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x14])],
            },
        }],
    }],
    related_formats: &[],
};
