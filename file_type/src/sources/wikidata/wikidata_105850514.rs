use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850514: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_514,
        source_type: SourceType::Wikidata,
        name: "Cinemaware music",
        extensions: &["cin"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x42, 0x4C, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
