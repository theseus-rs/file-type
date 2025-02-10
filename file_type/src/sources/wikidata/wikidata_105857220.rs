use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857220: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_220,
        source_type: SourceType::Wikidata,
        name: "Zebra2 Preset",
        extensions: &["h2p"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2F, 0x2A, 0x40, 0x6D, 0x65, 0x74, 0x61, 0x0A, 0x0A, 0x41, 0x75, 0x74,
                        0x68, 0x6F, 0x72, 0x3A, 0x0A, 0x27,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
