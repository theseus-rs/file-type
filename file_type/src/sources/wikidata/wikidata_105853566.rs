use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853566: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_566,
        source_type: SourceType::Wikidata,
        name: "Speculator '97 snapshot",
        extensions: &["zx82"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x58, 0x38, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
