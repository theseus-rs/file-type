use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854225: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_225,
        source_type: SourceType::Wikidata,
        name: "Maxis XA Audio (music)",
        extensions: &["xa"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x41, 0x4A, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
