use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858282: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_282,
        source_type: SourceType::Wikidata,
        name: "Eschalon Compressed font",
        extensions: &["ecmp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x43, 0x4D, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
