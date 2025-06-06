use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858487: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_487,
        source_type: SourceType::Wikidata,
        name: "bigWig Track Format",
        extensions: &["bigwig", "bw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x26, 0xFC, 0x8F, 0x88, 0x04, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
