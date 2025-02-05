use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857478: FileFormat = FileFormat {
    id: 105_857_478,
    source_type: SourceType::Wikidata,
    name: "3D Construction Kit Font",
    extensions: &["3fd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x45, 0x43, 0x4F, 0x4D, 0x50, 0x49, 0x4C, 0x45, 0x44, 0x20, 0x43, 0x52,
                    0x45, 0x41, 0x54, 0x49, 0x4F, 0x4E, 0x20, 0x46, 0x4F, 0x4E, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
