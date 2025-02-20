use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851581: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_581,
        source_type: SourceType::Wikidata,
        name: "Framework Terminal type",
        extensions: &["te"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1D, 0x0E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
