use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860824: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_824,
        source_type: SourceType::Wikidata,
        name: "RPG Maker data",
        extensions: &["rvdata2", "rxdata"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x04, 0x08])],
                },
            }],
        }],
        related_formats: &[],
    },
};
