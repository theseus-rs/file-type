use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857301: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_301,
        source_type: SourceType::Wikidata,
        name: "Seal Help format",
        extensions: &["hlp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x53, 0x48, 0x4C, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
