use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_76622426: FileType = FileType {
    file_format: &FileFormat {
        id: 76_622_426,
        source_type: SourceType::Wikidata,
        name: "Autodesk Alias 2017 Model",
        extensions: &["wire"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x8F, 0x53, 0x74, 0x75, 0x64, 0x69, 0x6F, 0x50, 0x61, 0x63, 0x6B, 0x65,
                        0x74, 0x46, 0x69, 0x6C, 0x65, 0x0A, 0x65, 0x6C, 0x69, 0x46, 0x74, 0x65,
                        0x6B, 0x63, 0x61, 0x50, 0x6F, 0x69, 0x64, 0x75, 0x74, 0x53, 0x0A, 0x31,
                        0x0A, 0x6D, 0x6F, 0x64, 0x65, 0x6C, 0x46, 0x69, 0x6C, 0x65, 0x0A, 0x40,
                        0x0A, 0x70, 0x72, 0x6F, 0x64, 0x75, 0x63, 0x74, 0x20, 0x41, 0x75, 0x74,
                        0x6F, 0x64, 0x65, 0x73, 0x6B, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
