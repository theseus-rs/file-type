use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857154: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_154,
        source_type: SourceType::Wikidata,
        name: "3D Studio 3.0 Help",
        extensions: &["hlp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x33, 0x44, 0x20, 0x53, 0x74, 0x75, 0x64, 0x69, 0x6F, 0x20, 0x33, 0x2E,
                        0x30, 0x20, 0x48, 0x65, 0x6C, 0x70, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x3A,
                        0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
