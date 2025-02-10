use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850642: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_642,
        source_type: SourceType::Wikidata,
        name: "Harvard Graphics Chart (v2.x)",
        extensions: &["cht"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x61, 0x6C, 0x63, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
