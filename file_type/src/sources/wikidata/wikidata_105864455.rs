use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864455: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_455,
        source_type: SourceType::Wikidata,
        name: "Password Commander Pro database (v2.x)",
        extensions: &["pwd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x43, 0x30, 0x32, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
