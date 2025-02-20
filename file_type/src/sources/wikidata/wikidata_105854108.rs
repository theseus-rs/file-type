use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854108: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_108,
        source_type: SourceType::Wikidata,
        name: "Alpha Four Script",
        extensions: &["scp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xBB, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
