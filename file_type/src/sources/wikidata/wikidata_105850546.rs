use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850546: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_546,
        source_type: SourceType::Wikidata,
        name: "World Construction Set Cloud map",
        extensions: &["cld"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x43, 0x53, 0x43, 0x6C, 0x6F, 0x75, 0x64, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
