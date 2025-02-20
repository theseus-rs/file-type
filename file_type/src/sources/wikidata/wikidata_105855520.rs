use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855520: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_520,
        source_type: SourceType::Wikidata,
        name: "Freeze compressed data (v2.x)",
        extensions: &["f"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1F, 0x9F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
