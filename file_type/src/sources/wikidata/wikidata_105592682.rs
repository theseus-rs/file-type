use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105592682: FileType = FileType {
    file_format: &FileFormat {
        id: 105_592_682,
        source_type: SourceType::Wikidata,
        name: "Power Tab",
        extensions: &["ptb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x70, 0x74, 0x61, 0x62])],
                },
            }],
        }],
        related_formats: &[],
    },
};
