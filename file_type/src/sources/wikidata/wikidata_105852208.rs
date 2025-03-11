use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852208: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_208,
        source_type: SourceType::Wikidata,
        name: "Unix-like shebang (var.1) (gen)",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x21, 0x2F, 0x75, 0x73, 0x72, 0x2F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
