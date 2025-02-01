use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27526733: FileFormat = FileFormat {
    id: 27_526_733,
    puid: "wikidata/27526733",
    name: "Graphics Interchange Format, version 87a",
    extensions: &["gif", "gif"],
    media_types: &["image/gif", "image/gif"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x49, 0x46, 0x38, 0x37, 0x61])],
            },
        }],
    }],
    related_formats: &[],
};
