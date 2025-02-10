use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855742: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_742,
        source_type: SourceType::Wikidata,
        name: "GEM-View Dither",
        extensions: &["dit"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x26, 0x57, 0x32, 0x35, 0x36, 0x00, 0x10, 0x00, 0x10,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
