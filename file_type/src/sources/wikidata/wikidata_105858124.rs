use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858124: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_124,
        source_type: SourceType::Wikidata,
        name: "APE ProSystem Atari 8-bit disk image (v2)",
        extensions: &["pro"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
