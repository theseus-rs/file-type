use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858660: FileFormat = FileFormat {
    id: 105_858_660,
    source_type: SourceType::Wikidata,
    name: "BaDonGo file info",
    extensions: &["badongo"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x23, 0x53, 0x54, 0x41, 0x52, 0x54, 0x20, 0x42, 0x41, 0x44, 0x4F, 0x4E,
                    0x47, 0x4F, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x0A, 0x54, 0x4F, 0x54, 0x41, 0x4C,
                    0x5F, 0x50, 0x41, 0x52, 0x54, 0x53, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
