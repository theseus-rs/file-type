use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853141: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_141,
        source_type: SourceType::Wikidata,
        name: "Sequence Alignment/Map format (with header)",
        extensions: &["sam"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40, 0x48, 0x44, 0x09, 0x56, 0x4E, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
