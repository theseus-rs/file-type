use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_921895: FileType = FileType {
    file_format: &FileFormat {
        id: 921_895,
        source_type: SourceType::Wikidata,
        name: "BBeB",
        extensions: &["lrf", "lrs", "lrx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x00, 0x52, 0x00, 0x46, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
