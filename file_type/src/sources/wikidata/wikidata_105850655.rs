use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850655: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_655,
        source_type: SourceType::Wikidata,
        name: "KCemu tape image",
        extensions: &["kct"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4B, 0x43, 0x65, 0x6D, 0x75, 0x20, 0x74, 0x61, 0x70, 0x65, 0x20, 0x66,
                        0x69, 0x6C, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
