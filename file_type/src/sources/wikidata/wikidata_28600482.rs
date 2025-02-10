use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28600482: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_482,
        source_type: SourceType::Wikidata,
        name: "DSK (Apple II)",
        extensions: &["dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0xA5, 0x27, 0xC9, 0x09, 0xD0])],
                },
            }],
        }],
        related_formats: &[],
    },
};
