use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867271: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_271,
        source_type: SourceType::Wikidata,
        name: "Eudora Address Book",
        extensions: &["nnt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x61, 0x6C, 0x69, 0x61, 0x73, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
