use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2295: FileFormat = FileFormat {
    id: 2_295,
    source_type: SourceType::Pronom,
    name: "Magic Shadow Archiver Disk Image File",
    extensions: &["msa"],
    media_types: &["application/vnd.msa-disk-image"],
    signatures: &[Signature {
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
