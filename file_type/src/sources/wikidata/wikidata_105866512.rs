use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866512: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_512,
        source_type: SourceType::Wikidata,
        name: "PhoneBook Backup",
        extensions: &["pbb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x45, 0x4C, 0x4C, 0x50, 0x42, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
