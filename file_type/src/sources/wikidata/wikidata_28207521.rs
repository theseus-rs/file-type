use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28207521: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_521,
        source_type: SourceType::Wikidata,
        name: "WPB",
        extensions: &["wpb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x50, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
