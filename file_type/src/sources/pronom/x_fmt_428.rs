use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_428: FileFormat = FileFormat {
    id: 815,
    puid: "x-fmt/428",
    name: "Microsoft Windows Shortcut",
    extensions: &["lnk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x00, 0x00, 0x00, 0x01, 0x14, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
