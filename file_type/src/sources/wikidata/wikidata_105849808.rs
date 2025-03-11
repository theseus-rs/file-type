use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849808: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_808,
        source_type: SourceType::Wikidata,
        name: "CauseWay Compressor compressed data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x57, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
