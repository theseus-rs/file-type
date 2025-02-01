use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1866: FileFormat = FileFormat {
    id: 2_720,
    puid: "fmt/1866",
    name: "Microsoft Powerpoint for Macintosh",
    extensions: &["ppt"],
    media_types: &["application/vnd.ms-PowerPoint"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0B, 0xAD, 0xDE, 0xED, 0x00, 0x00, 0x00, 0x02,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
