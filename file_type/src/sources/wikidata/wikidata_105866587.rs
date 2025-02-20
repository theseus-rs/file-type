use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866587: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_587,
        source_type: SourceType::Wikidata,
        name: "Clarion Project",
        extensions: &["prj"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2D, 0x2D, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
