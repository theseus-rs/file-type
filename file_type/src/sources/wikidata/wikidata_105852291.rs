use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852291: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_291,
        source_type: SourceType::Wikidata,
        name: "BeepFX Sound Effects Project",
        extensions: &["spj"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x6F, 0x75, 0x6E, 0x64, 0x45, 0x66, 0x66, 0x65, 0x63, 0x74, 0x73,
                        0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
