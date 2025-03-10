use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863414: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_414,
        source_type: SourceType::Wikidata,
        name: "MediaTek firmware",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4D, 0x4D, 0x01, 0x38, 0x00, 0x00, 0x00, 0x46, 0x49, 0x4C, 0x45,
                        0x5F, 0x49, 0x4E, 0x46, 0x4F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
