use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857851: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_851,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine compressed graphic (v1)",
        extensions: &["mos"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4F, 0x53, 0x43, 0x56, 0x31, 0x20, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
