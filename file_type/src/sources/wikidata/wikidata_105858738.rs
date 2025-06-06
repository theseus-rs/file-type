use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858738: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_738,
        source_type: SourceType::Wikidata,
        name: "AngelCode Bitmap Font Generator Configuration",
        extensions: &["bmfc"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x41, 0x6E, 0x67, 0x65, 0x6C, 0x43, 0x6F, 0x64, 0x65, 0x20,
                        0x42, 0x69, 0x74, 0x6D, 0x61, 0x70, 0x20, 0x46, 0x6F, 0x6E, 0x74, 0x20,
                        0x47, 0x65, 0x6E, 0x65, 0x72, 0x61, 0x74, 0x6F, 0x72, 0x20, 0x63, 0x6F,
                        0x6E, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20,
                        0x66, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
