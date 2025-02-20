use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853063: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_063,
        source_type: SourceType::Wikidata,
        name: "Super Data Format",
        extensions: &["sdf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x41, 0x42, 0x4C, 0x45])],
                },
            }],
        }],
        related_formats: &[],
    },
};
