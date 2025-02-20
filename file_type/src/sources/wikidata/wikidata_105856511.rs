use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856511: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_511,
        source_type: SourceType::Wikidata,
        name: "WinPlot data (v2)",
        extensions: &["wp2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x03, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
