use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852253: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_253,
        source_type: SourceType::Wikidata,
        name: "sPlan 7.0 schematic",
        extensions: &["spl7"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x07, 0x53, 0x50, 0x4C, 0x41, 0x4E, 0x37, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
