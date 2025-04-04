use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863146: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_146,
        source_type: SourceType::Wikidata,
        name: "MyPhoneExplorer Backup",
        extensions: &["mpb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0x4D, 0x79, 0x50, 0x68, 0x6F, 0x6E, 0x65, 0x45, 0x78, 0x70, 0x6C,
                        0x6F, 0x72, 0x65, 0x72, 0x5F, 0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74,
                        0x49, 0x44, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
