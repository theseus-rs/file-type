use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855249: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_249,
        source_type: SourceType::Wikidata,
        name: "Origin Fitting Function Definition File",
        extensions: &["fdf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
