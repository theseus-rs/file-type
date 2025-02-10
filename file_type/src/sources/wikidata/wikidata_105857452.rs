use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857452: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_452,
        source_type: SourceType::Wikidata,
        name: "Shaper LUT",
        extensions: &["3dl"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x33, 0x44, 0x4D, 0x45, 0x53, 0x48])],
                },
            }],
        }],
        related_formats: &[],
    },
};
