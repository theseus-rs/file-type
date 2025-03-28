use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861876: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_876,
        source_type: SourceType::Wikidata,
        name: "Magic Workstation Deck",
        extensions: &["mwdeck"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2F, 0x2F, 0x20, 0x44, 0x65, 0x63, 0x6B, 0x20, 0x66, 0x69, 0x6C, 0x65,
                        0x20, 0x66, 0x6F, 0x72, 0x20, 0x4D, 0x61, 0x67, 0x69, 0x63, 0x20, 0x57,
                        0x6F, 0x72, 0x6B, 0x73, 0x74, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
