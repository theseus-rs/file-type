use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864645: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_645,
        source_type: SourceType::Wikidata,
        name: "PICkit 2 firmware",
        extensions: &["pk2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x32, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
