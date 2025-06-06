use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857487: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_487,
        source_type: SourceType::Wikidata,
        name: "3-D Professional lathe plate",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x53, 0x4C, 0x41, 0x54, 0x48, 0x45, 0x50, 0x4C, 0x41, 0x54, 0x45,
                        0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
