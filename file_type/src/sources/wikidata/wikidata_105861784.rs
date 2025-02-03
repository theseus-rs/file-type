use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861784: FileFormat = FileFormat {
    id: 105_861_784,
    source_type: SourceType::Wikidata,
    name: "MAIET Encrypted File System",
    extensions: &["mef"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x41, 0x49, 0x45, 0x54, 0x20, 0x45, 0x6E, 0x63, 0x72, 0x79, 0x70, 0x74,
                    0x65, 0x64, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x53, 0x79, 0x73, 0x74, 0x65,
                    0x6D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
