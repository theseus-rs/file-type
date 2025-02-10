use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865949: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_949,
        source_type: SourceType::Wikidata,
        name: "Project Master Plan data",
        extensions: &["pln"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x43, 0x52, 0x45, 0x45, 0x4E, 0x4E, 0x4F, 0x44, 0x45, 0x53, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
