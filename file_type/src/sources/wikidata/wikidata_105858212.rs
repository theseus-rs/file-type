use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858212: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_212,
        source_type: SourceType::Wikidata,
        name: "Compucon EOS Design Format",
        extensions: &["erf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6F, 0x6D, 0x70, 0x75, 0x63, 0x6F, 0x6E, 0x20, 0x45, 0x4F, 0x53,
                        0x20, 0x44, 0x65, 0x73, 0x69, 0x67, 0x6E, 0x20, 0x46, 0x69, 0x6C, 0x65,
                        0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
