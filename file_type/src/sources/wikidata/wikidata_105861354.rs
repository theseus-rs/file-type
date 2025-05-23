use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861354: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_354,
        source_type: SourceType::Wikidata,
        name: "Logo Report View File",
        extensions: &["lvf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x6F, 0x67, 0x6F, 0x20, 0x52, 0x65, 0x70, 0x6F, 0x72, 0x74, 0x20,
                        0x56, 0x69, 0x65, 0x77, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
