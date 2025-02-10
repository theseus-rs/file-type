use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866166: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_166,
        source_type: SourceType::Wikidata,
        name: "Windows 98 passwords",
        extensions: &["pwl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xE3, 0x82, 0x85, 0x96])],
                },
            }],
        }],
        related_formats: &[],
    },
};
