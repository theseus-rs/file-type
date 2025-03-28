use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852322: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_322,
        source_type: SourceType::Wikidata,
        name: "Slim Show project",
        extensions: &["ss"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x73, 0x70, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
