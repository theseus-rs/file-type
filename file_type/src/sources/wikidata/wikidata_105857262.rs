use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857262: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_262,
        source_type: SourceType::Wikidata,
        name: "Hamic worksheet",
        extensions: &["hmc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x48, 0x4D, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
