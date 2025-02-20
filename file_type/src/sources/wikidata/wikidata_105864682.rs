use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864682: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_682,
        source_type: SourceType::Wikidata,
        name: "PDT structure definition",
        extensions: &["pdt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x44, 0x54, 0x2E, 0x53, 0x54, 0x52, 0x55, 0x43, 0x54, 0x55, 0x52,
                        0x45, 0x20, 0x20, 0xB3, 0x20, 0x44, 0x6F, 0x20, 0x4E, 0x4F, 0x54, 0x20,
                        0x6D, 0x6F, 0x76, 0x65, 0x20, 0x6F, 0x72, 0x20, 0x63, 0x68, 0x61, 0x6E,
                        0x67, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6C, 0x69, 0x6E, 0x65,
                        0x21, 0x20, 0xB3, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
