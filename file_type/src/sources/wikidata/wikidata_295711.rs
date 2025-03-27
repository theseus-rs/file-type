use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_295711: FileType = FileType {
    file_format: &FileFormat {
        id: 295_711,
        source_type: SourceType::Wikidata,
        name: "Windows Animated Cursor",
        extensions: &["ani"],
        media_types: &["application/x-navi-animation"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x43, 0x4F, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
