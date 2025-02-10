use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855625: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_625,
        source_type: SourceType::Wikidata,
        name: "EPOC OPL source",
        extensions: &["oph", "opl", "oxh"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x37, 0x00, 0x00, 0x10, 0x6D, 0x00, 0x00, 0x10, 0x85, 0x00, 0x00, 0x10,
                        0xF4,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
