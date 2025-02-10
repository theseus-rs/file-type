use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854546: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_546,
        source_type: SourceType::Wikidata,
        name: "MediaZip compressed archive",
        extensions: &["mzf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x4D, 0x5A, 0x46, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
