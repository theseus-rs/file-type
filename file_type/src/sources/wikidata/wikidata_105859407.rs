use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859407: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_407,
        source_type: SourceType::Wikidata,
        name: "Q-emulator QDOS file header",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5D, 0x21, 0x51, 0x44, 0x4F, 0x53, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20,
                        0x48, 0x65, 0x61, 0x64, 0x65, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
