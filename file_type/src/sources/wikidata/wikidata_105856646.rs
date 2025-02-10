use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856646: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_646,
        source_type: SourceType::Wikidata,
        name: "AS-EASY-AS Worksheet (generic)",
        extensions: &["wks"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x02, 0x00, 0x04, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
