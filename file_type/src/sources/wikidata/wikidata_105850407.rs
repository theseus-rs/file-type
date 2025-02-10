use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850407: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_407,
        source_type: SourceType::Wikidata,
        name: "Cabbage script",
        extensions: &["csd"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x43, 0x61, 0x62, 0x62, 0x61, 0x67, 0x65, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
