use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857169: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_169,
        source_type: SourceType::Wikidata,
        name: "OS/2 Help",
        extensions: &["hlp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x53, 0x50, 0x10, 0x9B, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
