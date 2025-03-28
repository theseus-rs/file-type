use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850969: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_969,
        source_type: SourceType::Wikidata,
        name: "Sony Ericsson Theme (for mobile phones)",
        extensions: &["thm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x75, 0x73, 0x74, 0x61, 0x72])],
                },
            }],
        }],
        related_formats: &[],
    },
};
