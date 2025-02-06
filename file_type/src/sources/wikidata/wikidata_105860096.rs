use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860096: FileFormat = FileFormat {
    id: 105_860_096,
    source_type: SourceType::Wikidata,
    name: "Vissim 3D Vehicle model",
    extensions: &["v3d"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x20, 0x56, 0x49, 0x53, 0x53, 0x49, 0x4D, 0x20, 0x33, 0x44, 0x20, 0x4D, 0x6F,
                    0x64, 0x65, 0x6C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
