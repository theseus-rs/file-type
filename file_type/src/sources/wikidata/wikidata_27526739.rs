use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27526739: FileFormat = FileFormat {
    id: 27_526_739,
    puid: "wikidata/27526739",
    name: "Graphics Interchange Format, version 89a",
    extensions: &["gif", "gif"],
    media_types: &["image/gif", "image/gif"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x49, 0x46, 0x38, 0x39, 0x61])],
            },
        }],
    }],
    related_formats: &[],
};
