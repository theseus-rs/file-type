use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864770: FileFormat = FileFormat {
    id: 105_864_770,
    puid: "wikidata/105864770",
    name: "Palm Diddle sketch drawing",
    extensions: &["pdb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x49, 0x44, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
