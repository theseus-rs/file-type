use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855254: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_254,
        source_type: SourceType::Wikidata,
        name: "FreePCB Library",
        extensions: &["fpl"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6E, 0x61, 0x6D, 0x65, 0x3A, 0x20, 0x22])],
                },
            }],
        }],
        related_formats: &[],
    },
};
