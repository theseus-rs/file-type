use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849826: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_826,
        source_type: SourceType::Wikidata,
        name: "Chile compressed file",
        extensions: &["chl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x48, 0x4C, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
