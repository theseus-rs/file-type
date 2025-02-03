use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859891: FileFormat = FileFormat {
    id: 105_859_891,
    source_type: SourceType::Wikidata,
    name: "Voxel Animation",
    extensions: &["vxl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x6F, 0x78, 0x65, 0x6C, 0x20, 0x41, 0x6E, 0x69, 0x6D, 0x61, 0x74, 0x69,
                    0x6F, 0x6E, 0x00, 0x01, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
