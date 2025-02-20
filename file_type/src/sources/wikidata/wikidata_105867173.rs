use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867173: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_173,
        source_type: SourceType::Wikidata,
        name: "the Word Encrypted New Testament Text Module",
        extensions: &["ntx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x57, 0x45, 0x4E, 0x43, 0x42, 0x4D, 0x4F, 0x44, 0x01, 0x01, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
