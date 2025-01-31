use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867574: FileFormat = FileFormat {
    id: 105_867_574,
    puid: "wikidata/105867574",
    name: "NAPLPS graphics",
    extensions: &["nap"],
    media_types: &["image/naplps"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x18, 0x1B, 0x22, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
