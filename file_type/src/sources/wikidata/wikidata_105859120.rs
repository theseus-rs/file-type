use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859120: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_120,
        source_type: SourceType::Wikidata,
        name: "PM XV bitmap (alt.endianess)",
        extensions: &["pm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x45, 0x49, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
