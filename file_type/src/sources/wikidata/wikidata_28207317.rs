use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207317: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_317,
        source_type: SourceType::Wikidata,
        name: "Standard Archive Format",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x64, 0x53, 0x69, 0x7A, 0x65, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
