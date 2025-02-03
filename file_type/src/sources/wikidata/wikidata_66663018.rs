use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66663018: FileFormat = FileFormat {
    id: 66_663_018,
    source_type: SourceType::Wikidata,
    name: "Lotus Freelance Presentation",
    extensions: &["prz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x01, 0x0F, 0x00, 0x46, 0x52, 0x45, 0x45, 0x4C, 0x41, 0x4E, 0x43, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
