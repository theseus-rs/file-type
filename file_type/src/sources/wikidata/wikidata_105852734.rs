use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852734: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_734,
        source_type: SourceType::Wikidata,
        name: "Session Description Protocol",
        extensions: &["sdp"],
        media_types: &["application/sdp"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x76, 0x3D, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
