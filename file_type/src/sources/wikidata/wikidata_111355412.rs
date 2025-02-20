use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111355412: FileType = FileType {
    file_format: &FileFormat {
        id: 111_355_412,
        source_type: SourceType::Wikidata,
        name: "Panasonic voice file",
        extensions: &["vm1"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x61, 0x6E, 0x61, 0x73, 0x6F, 0x6E, 0x69, 0x63, 0x20, 0x53, 0x44,
                        0x20, 0x56, 0x6F, 0x69, 0x63, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
