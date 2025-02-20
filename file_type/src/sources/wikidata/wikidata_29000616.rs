use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000616: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_616,
        source_type: SourceType::Wikidata,
        name: "Google Chrome Extension",
        extensions: &["crx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x72, 0x32, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
