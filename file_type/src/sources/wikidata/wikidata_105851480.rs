use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851480: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_480,
        source_type: SourceType::Wikidata,
        name: "TeamViewer Session",
        extensions: &["tvs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x65, 0x61, 0x6D, 0x56, 0x69, 0x65, 0x77, 0x65, 0x72, 0x20, 0x53,
                        0x65, 0x73, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x0D,
                        0x0A, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
