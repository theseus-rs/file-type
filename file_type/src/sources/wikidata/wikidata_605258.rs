use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_605258: FileType = FileType {
    file_format: &FileFormat {
        id: 605_258,
        source_type: SourceType::Wikidata,
        name: "Gerber format",
        extensions: &["gbr"],
        media_types: &["application/vnd.gerber"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25])],
                },
            }],
        }],
        related_formats: &[],
    },
};
