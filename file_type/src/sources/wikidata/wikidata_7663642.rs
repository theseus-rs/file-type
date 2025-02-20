use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7663642: FileType = FileType {
    file_format: &FileFormat {
        id: 7_663_642,
        source_type: SourceType::Wikidata,
        name: "System Deployment Image",
        extensions: &["sdi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x24, 0x53, 0x44, 0x49, 0x30, 0x30, 0x30, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
