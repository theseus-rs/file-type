use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853837: FileFormat = FileFormat {
    id: 105_853_837,
    source_type: SourceType::Wikidata,
    name: "Psion S3a/3c/Siena Agenda",
    extensions: &["agn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x67, 0x65, 0x6E, 0x64, 0x61, 0x46, 0x69, 0x6C, 0x65, 0x54, 0x79, 0x70,
                    0x65, 0x2A, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
