use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852076: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_076,
        source_type: SourceType::Wikidata,
        name: "Dynamic Publisher Stamp",
        extensions: &["stp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x79, 0x6E, 0x61, 0x6D, 0x69, 0x63, 0x20, 0x50, 0x75, 0x62, 0x6C,
                        0x69, 0x73, 0x68, 0x65, 0x72, 0x20, 0x53, 0x74, 0x61, 0x6D, 0x70,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
