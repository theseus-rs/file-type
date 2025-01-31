use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1472: FileFormat = FileFormat {
    id: 2_295,
    puid: "fmt/1472",
    name: "Magic Shadow Archiver Disk Image File",
    extensions: &["msa"],
    media_types: &["application/vnd.msa-disk-image"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0E, 0x0F])],
            },
        }],
    }],
    related_formats: &[],
};
