use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857364: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_364,
        source_type: SourceType::Wikidata,
        name: "Jw_cad data",
        extensions: &["jwc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6A, 0x77, 0x5F, 0x63, 0x61, 0x64, 0x28, 0x63, 0x29, 0x64, 0x61, 0x74,
                        0x61, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
