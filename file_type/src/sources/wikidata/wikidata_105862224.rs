use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862224: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_224,
        source_type: SourceType::Wikidata,
        name: "EPICS sscan/saveData format",
        extensions: &["mda"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3F, 0xA6, 0x66, 0x66, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
