use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857165: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_165,
        source_type: SourceType::Wikidata,
        name: "HarmonyCSV format",
        extensions: &["csv"],
        media_types: &["text/csv"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x61, 0x72, 0x6D, 0x6F, 0x6E, 0x79, 0x43, 0x53, 0x56, 0x20, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
