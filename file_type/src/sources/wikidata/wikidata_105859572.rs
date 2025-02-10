use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859572: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_572,
        source_type: SourceType::Wikidata,
        name: "MUX video",
        extensions: &["mux"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x07, 0xAA])],
                },
            }],
        }],
        related_formats: &[],
    },
};
