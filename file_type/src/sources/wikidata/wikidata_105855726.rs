use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855726: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_726,
        source_type: SourceType::Wikidata,
        name: "DVR-Studio stream",
        extensions: &["dvr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x56, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
