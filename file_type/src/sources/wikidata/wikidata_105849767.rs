use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849767: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_767,
        source_type: SourceType::Wikidata,
        name: "Cryptx encrypted data",
        extensions: &["crypt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x52, 0x59, 0x50, 0x54, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
