use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762684: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_684,
        source_type: SourceType::Wikidata,
        name: "WinArchiver Extended Disc Image",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x44, 0x49, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
