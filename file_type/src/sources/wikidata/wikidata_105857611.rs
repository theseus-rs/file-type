use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857611: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_611,
        source_type: SourceType::Wikidata,
        name: "iMovie Project",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x69, 0x4D, 0x6F, 0x76, 0x69, 0x65, 0x20, 0x50, 0x72, 0x6F, 0x6A, 0x65,
                        0x63, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
