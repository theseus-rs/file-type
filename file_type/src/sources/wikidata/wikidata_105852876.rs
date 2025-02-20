use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852876: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_876,
        source_type: SourceType::Wikidata,
        name: "Smart Software Time Manager Data",
        extensions: &["dtm", "stm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x63, 0x68, 0x75, 0x63, 0x6B, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
