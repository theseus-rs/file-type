use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1904: FileFormat = FileFormat {
    id: 2_760,
    puid: "fmt/1904",
    name: "Pasti Floppy Disk Image",
    extensions: &["stx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x53, 0x59])],
            },
        }],
    }],
    related_formats: &[],
};
