use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852499: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_499,
        source_type: SourceType::Wikidata,
        name: "SCC Blaffer NT PSG Instrument",
        extensions: &["sbp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x6C, 0x61, 0x66, 0x20, 0x4E, 0x54, 0x20, 0x50, 0x53, 0x47, 0x20,
                        0x49, 0x6E, 0x73, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
