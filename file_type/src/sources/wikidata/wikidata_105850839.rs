use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850839: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_839,
        source_type: SourceType::Wikidata,
        name: "KOLEKO Save state",
        extensions: &["ksv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x31, 0xB9, 0x73, 0xC3, 0x6E, 0x00, 0xFF, 0xFF,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
