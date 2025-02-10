use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28346532: FileType = FileType {
    file_format: &FileFormat {
        id: 28_346_532,
        source_type: SourceType::Wikidata,
        name: "Amiga bitmap font",
        extensions: &["font"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0F, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
