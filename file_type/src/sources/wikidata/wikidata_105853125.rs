use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853125: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_125,
        source_type: SourceType::Wikidata,
        name: "SuperTux World Map",
        extensions: &["stwm"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x28, 0x73, 0x75, 0x70, 0x65, 0x72, 0x74, 0x75, 0x78, 0x2D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
