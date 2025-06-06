use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864144: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_144,
        source_type: SourceType::Wikidata,
        name: "WinMorph project",
        extensions: &["mrf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x20, 0x57, 0x69, 0x6E, 0x4D, 0x6F, 0x72, 0x70, 0x68, 0x20, 0x67,
                        0x65, 0x6E, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x20, 0x70, 0x72, 0x6F,
                        0x6A, 0x65, 0x63, 0x74, 0x2E, 0x20, 0x44, 0x6F, 0x20, 0x6E, 0x6F, 0x74,
                        0x20, 0x65, 0x64, 0x69, 0x74, 0x20, 0x6D, 0x61, 0x6E, 0x75, 0x61, 0x6C,
                        0x6C, 0x79, 0x21, 0x21, 0x20, 0x5D, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
