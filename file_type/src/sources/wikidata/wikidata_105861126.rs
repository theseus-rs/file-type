use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861126: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_126,
        source_type: SourceType::Wikidata,
        name: "JAR Index",
        extensions: &["list"],
        media_types: &["package file format"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4A, 0x61, 0x72, 0x49, 0x6E, 0x64, 0x65, 0x78, 0x2D, 0x56, 0x65, 0x72,
                        0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x20, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
