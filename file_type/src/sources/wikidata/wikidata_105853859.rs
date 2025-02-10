use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853859: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_859,
        source_type: SourceType::Wikidata,
        name: "ARX compressed archive",
        extensions: &["arx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2D, 0x6C, 0x68, 0x31, 0x2D, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
