use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862720: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_720,
        source_type: SourceType::Wikidata,
        name: "iOS backup manifest index",
        extensions: &["mbdx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6D, 0x62, 0x64, 0x78, 0x02, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
