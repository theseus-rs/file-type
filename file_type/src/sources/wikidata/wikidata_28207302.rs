use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207302: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_302,
        source_type: SourceType::Wikidata,
        name: "True Colour Sprite",
        extensions: &["trs"],
        media_types: &["application/octet-stream", "image/x-spooky-sprites"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x43, 0x53, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
