use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851759: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_759,
        source_type: SourceType::Wikidata,
        name: "STereoLithography (binary) (alt2) (gen)",
        extensions: &["stl"],
        media_types: &["model/x.stl-binary"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x6F, 0x6C, 0x69, 0x64, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
