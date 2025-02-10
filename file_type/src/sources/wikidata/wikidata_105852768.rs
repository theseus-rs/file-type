use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852768: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_768,
        source_type: SourceType::Wikidata,
        name: "SevenUp bitmap",
        extensions: &["sev"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x65, 0x76, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
