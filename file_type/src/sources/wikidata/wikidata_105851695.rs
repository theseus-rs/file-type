use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851695: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_695,
        source_type: SourceType::Wikidata,
        name: "Windows Live Messenger Log file",
        extensions: &["sqm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x53, 0x51, 0x4D, 0x78])],
                },
            }],
        }],
        related_formats: &[],
    },
};
