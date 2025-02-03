use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_73624420: FileFormat = FileFormat {
    id: 73_624_420,
    source_type: SourceType::Wikidata,
    name: "Quick 3D Cover project",
    extensions: &["q3c"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x51, 0x32, 0x44, 0x44, 0x65, 0x73, 0x69, 0x67, 0x6E, 0x30, 0x30, 0x30,
                    0x30, 0x30, 0x30, 0x5D, 0x0D, 0x0A, 0x56, 0x65, 0x72, 0x3D, 0x31, 0x30, 0x30,
                    0x0D, 0x0A, 0x49, 0x64, 0x3D, 0x30, 0x0D, 0x0A, 0x4E, 0x61, 0x6D, 0x65, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
