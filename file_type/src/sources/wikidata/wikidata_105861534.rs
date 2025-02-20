use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861534: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_534,
        source_type: SourceType::Wikidata,
        name: "LAS sidecar auxilliary data",
        extensions: &["lasx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x41, 0x53, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
