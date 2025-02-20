use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850542: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_542,
        source_type: SourceType::Wikidata,
        name: "Blackberry Binary Executable",
        extensions: &["cod"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xDE, 0xC0, 0xFF, 0xFF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
