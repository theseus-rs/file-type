use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2151: FileFormat = FileFormat {
    id: 2_151,
    source_type: SourceType::Pronom,
    name: "Sony PictureGear Studio PhotoAlbum",
    extensions: &["amu", "amd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x10, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x01]),
                    Token::WildcardCountRange(0, 288),
                    Token::Literal(&[0x42, 0x4D, 0xB6, 0xBB]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
