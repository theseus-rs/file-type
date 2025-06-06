use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852362: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_362,
        source_type: SourceType::Wikidata,
        name: "Greenstreet Publisher snippet",
        extensions: &["srp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x43, 0x52, 0x50, 0x00, 0x00, 0x00, 0x02, 0x40, 0xDF, 0x44, 0x53,
                        0x54, 0x59,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
