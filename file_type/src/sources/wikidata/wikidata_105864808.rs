use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864808: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_808,
        source_type: SourceType::Wikidata,
        name: "Photogenics matrix data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x54, 0x52, 0x58, 0x0A, 0x09])],
                },
            }],
        }],
        related_formats: &[],
    },
};
