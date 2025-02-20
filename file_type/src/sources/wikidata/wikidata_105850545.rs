use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850545: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_545,
        source_type: SourceType::Wikidata,
        name: "Javelin Country Driver",
        extensions: &["cdv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4A, 0x61, 0x76, 0x65, 0x6C, 0x69, 0x6E, 0x43, 0x6F, 0x75, 0x6E, 0x74,
                        0x72, 0x79, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
