use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851814: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_814,
        source_type: SourceType::Wikidata,
        name: "PowerBASIC Static Link Library",
        extensions: &["pblib", "sll"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x45, 0x01, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
