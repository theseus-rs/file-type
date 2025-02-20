use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851040: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_040,
        source_type: SourceType::Wikidata,
        name: "RTFGEN Topic data",
        extensions: &["tpc"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5C, 0x64, 0x6F, 0x63, 0x73, 0x74, 0x61, 0x72, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
