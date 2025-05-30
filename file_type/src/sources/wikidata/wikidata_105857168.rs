use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857168: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_168,
        source_type: SourceType::Wikidata,
        name: "Health Level-7 data (pipe delimited)",
        extensions: &["hl7"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x53, 0x48, 0x7C, 0x5E, 0x7E, 0x5C, 0x26, 0x7C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
