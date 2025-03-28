use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857299: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_299,
        source_type: SourceType::Wikidata,
        name: "BDC HelpSystem Help info",
        extensions: &["hlp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x44, 0x43, 0x20, 0x48, 0x65, 0x6C, 0x70, 0x53, 0x79, 0x73, 0x74,
                        0x65, 0x6D, 0x20, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
