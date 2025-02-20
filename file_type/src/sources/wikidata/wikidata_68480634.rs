use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_68480634: FileType = FileType {
    file_format: &FileFormat {
        id: 68_480_634,
        source_type: SourceType::Wikidata,
        name: "Magic: The Gathering cards Deck file format",
        extensions: &["dec"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2F, 0x2F, 0x4E, 0x41, 0x4D, 0x45, 0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
