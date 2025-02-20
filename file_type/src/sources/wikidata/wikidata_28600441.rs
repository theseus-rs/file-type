use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600441: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_441,
        source_type: SourceType::Wikidata,
        name: "CrLZH",
        extensions: &["?y?", "yyy"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x76, 0xFD])],
                },
            }],
        }],
        related_formats: &[],
    },
};
