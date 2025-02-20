use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862277: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_277,
        source_type: SourceType::Wikidata,
        name: "Room Arranger design",
        extensions: &["mst"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x52, 0x6F, 0x6F, 0x6D, 0x5D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
