use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28919062: FileFormat = FileFormat {
    id: 28_919_062,
    source_type: SourceType::Wikidata,
    name: "MacCaption VANC",
    extensions: &["mcc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x69, 0x6C, 0x65, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x3D, 0x4D,
                    0x61, 0x63, 0x43, 0x61, 0x70, 0x74, 0x69, 0x6F, 0x6E, 0x5F, 0x4D, 0x43, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
