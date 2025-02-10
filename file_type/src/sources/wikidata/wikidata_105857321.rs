use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857321: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_321,
        source_type: SourceType::Wikidata,
        name: "I.E.S. HyperText",
        extensions: &["hyp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x03, 0x48, 0x04, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
