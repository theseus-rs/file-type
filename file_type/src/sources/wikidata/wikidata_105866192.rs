use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866192: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_192,
        source_type: SourceType::Wikidata,
        name: "Windows 95 passwords",
        extensions: &["pwl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xB0, 0x4D, 0x46, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
