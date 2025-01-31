use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_341: FileFormat = FileFormat {
    id: 1_086,
    puid: "fmt/341",
    name: "Macintosh PICT Image",
    extensions: &["pct", "pict", "pic"],
    media_types: &["image/x-pict"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(522),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x11, 0x02, 0xFF, 0x0C, 0x00])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 122,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
