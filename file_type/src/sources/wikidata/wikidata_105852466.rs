use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852466: FileFormat = FileFormat {
    id: 105_852_466,
    source_type: SourceType::Wikidata,
    name: "Software Ideas Modeler Style Set",
    extensions: &["simss"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x3C, 0x73, 0x69, 0x6D, 0x2D, 0x73, 0x74, 0x79, 0x6C, 0x65,
                    0x2D, 0x73, 0x65, 0x74, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D,
                    0x22, 0x31, 0x2E, 0x30, 0x22, 0x20, 0x6E, 0x61, 0x6D, 0x65, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
