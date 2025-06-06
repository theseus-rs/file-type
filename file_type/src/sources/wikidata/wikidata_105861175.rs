use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861175: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_175,
        source_type: SourceType::Wikidata,
        name: "MapWindow Labels",
        extensions: &["lbl"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x4D, 0x61, 0x70, 0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
