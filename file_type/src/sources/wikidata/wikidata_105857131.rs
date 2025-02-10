use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857131: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_131,
        source_type: SourceType::Wikidata,
        name: "GNU Gettext Machine Object (big endian)",
        extensions: &["gmo", "mo"],
        media_types: &["application/x-gettext-translation"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x95, 0x04, 0x12, 0xDE])],
                },
            }],
        }],
        related_formats: &[],
    },
};
