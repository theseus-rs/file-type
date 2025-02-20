use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849606: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_606,
        source_type: SourceType::Wikidata,
        name: "KiCad Drawing",
        extensions: &["cad"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x24, 0x48, 0x45, 0x41, 0x44, 0x45, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
