use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1243: FileFormat = FileFormat {
    id: 2_061,
    puid: "fmt/1243",
    name: "ZFO (Message) File",
    extensions: &["zfo"],
    media_types: &["application/vnd.software602.filler.form-xml-zip"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x69, 0x73, 0x64, 0x73, 0x2E, 0x63,
                    0x7A, 0x65, 0x63, 0x68, 0x70, 0x6F, 0x69, 0x6E, 0x74, 0x2E, 0x63, 0x7A, 0x2F,
                    0x76, 0x32, 0x30, 0x2F, 0x6D, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_469,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
