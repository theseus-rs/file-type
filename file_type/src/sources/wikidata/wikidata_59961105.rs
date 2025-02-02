use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59961105: FileFormat = FileFormat {
    id: 59_961_105,
    source_type: SourceType::Wikidata,
    name: "InstallShield Compiled Rules File",
    extensions: &["inx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x61, 0x4C, 0x75, 0x5A, 0x00, 0x00, 0x43, 0x6F, 0x70, 0x79, 0x72, 0x69, 0x67,
                    0x68, 0x74, 0x20, 0x28, 0x63, 0x29, 0x20, 0x31, 0x39, 0x39, 0x30, 0x2D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
