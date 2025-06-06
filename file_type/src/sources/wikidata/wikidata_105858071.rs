use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858071: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_071,
        source_type: SourceType::Wikidata,
        name: "Lhwarp compressed disk image",
        extensions: &["lhw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x03, 0x09, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
