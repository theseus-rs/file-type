use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855104: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_104,
        source_type: SourceType::Wikidata,
        name: "QuArk compressed archive",
        extensions: &["ark"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x37, 0x04, 0x10, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
