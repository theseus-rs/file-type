use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67206686: FileFormat = FileFormat {
    id: 67_206_686,
    source_type: SourceType::Wikidata,
    name: "FastCAD Windows file",
    extensions: &["fcw"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x43, 0x57, 0x20, 0x28, 0x46, 0x61, 0x73, 0x74, 0x43, 0x41, 0x44, 0x20,
                    0x66, 0x6F, 0x72, 0x20, 0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73, 0x29, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
