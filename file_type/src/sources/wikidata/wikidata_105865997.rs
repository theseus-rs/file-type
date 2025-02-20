use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865997: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_997,
        source_type: SourceType::Wikidata,
        name: "Programming Object File",
        extensions: &["pof"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x4F, 0x46, 0x00, 0x00, 0x00, 0x01, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
