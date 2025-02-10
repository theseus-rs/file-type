use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853696: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_696,
        source_type: SourceType::Wikidata,
        name: "CAZIPXP compressed archive",
        extensions: &["caz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x41, 0x5A, 0x49, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
