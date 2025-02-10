use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850500: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_500,
        source_type: SourceType::Wikidata,
        name: "Snoop capture file",
        extensions: &["cap"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x6E, 0x6F, 0x6F, 0x70])],
                },
            }],
        }],
        related_formats: &[],
    },
};
