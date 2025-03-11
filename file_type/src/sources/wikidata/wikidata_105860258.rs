use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860258: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_258,
        source_type: SourceType::Wikidata,
        name: "Emulated RX01 disk image",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x72, 0x78, 0x30, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
