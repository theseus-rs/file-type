use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857238: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_238,
        source_type: SourceType::Wikidata,
        name: "Hard Disk Menu System menu",
        extensions: &["000", "999"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x44, 0x4D, 0x53, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
