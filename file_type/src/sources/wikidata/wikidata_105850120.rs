use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850120: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_120,
        source_type: SourceType::Wikidata,
        name: "CorelDRAW drawing (zipped)",
        extensions: &["cdr"],
        media_types: &["application/x-coreldraw"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
