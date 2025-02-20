use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207131: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_131,
        source_type: SourceType::Wikidata,
        name: "Prism Paint",
        extensions: &["pnt", "tpi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4E, 0x54, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
