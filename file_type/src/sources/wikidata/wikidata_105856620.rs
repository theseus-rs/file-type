use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856620: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_620,
        source_type: SourceType::Wikidata,
        name: "WinPlot data (v3)",
        extensions: &["wp3"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xC1, 0x03, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
