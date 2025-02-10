use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856950: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_950,
        source_type: SourceType::Wikidata,
        name: "Micro Focus COBOL generated code",
        extensions: &["gnt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x69, 0x63, 0x72, 0x6F, 0x20, 0x46, 0x6F, 0x63, 0x75, 0x73, 0x20,
                        0x43, 0x4F, 0x42, 0x4F, 0x4C, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
