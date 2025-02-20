use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852470: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_470,
        source_type: SourceType::Wikidata,
        name: "BinkleyTerm Schedule data",
        extensions: &["scd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x69, 0x6E, 0x6B, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6C, 0x65,
                        0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
