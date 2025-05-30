use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855387: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_387,
        source_type: SourceType::Wikidata,
        name: "Guild Wars gamedata",
        extensions: &["ffna"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x66, 0x66, 0x6E, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
