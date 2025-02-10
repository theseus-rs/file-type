use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859768: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_768,
        source_type: SourceType::Wikidata,
        name: "CRYO HNM4 video",
        extensions: &["hnm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x4E, 0x4D, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
