use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857906: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_906,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine Store (v9.0)",
        extensions: &["sto"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x54, 0x4F, 0x52, 0x56, 0x39, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
