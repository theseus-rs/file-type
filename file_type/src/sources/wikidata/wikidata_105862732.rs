use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862732: FileFormat = FileFormat {
    id: 105_862_732,
    source_type: SourceType::Wikidata,
    name: "MMFW Pictures",
    extensions: &["mmp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x4D, 0x46, 0x57, 0x20, 0x50, 0x69, 0x63, 0x74, 0x75, 0x72, 0x65, 0x73,
                    0x00, 0x00, 0x00, 0x4D, 0x4D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
