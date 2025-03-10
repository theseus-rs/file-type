use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852883: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_883,
        source_type: SourceType::Wikidata,
        name: "Installer VISE Mac package",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x56, 0x43, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
