use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857005: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_005,
        source_type: SourceType::Wikidata,
        name: "SBG Object Text",
        extensions: &["geo"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x69, 0x6C, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
