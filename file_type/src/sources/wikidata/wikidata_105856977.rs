use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856977: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_977,
        source_type: SourceType::Wikidata,
        name: "GNU Gettext Machine Object (litte endian)",
        extensions: &["gmo", "mo"],
        media_types: &["application/x-gettext-translation"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xDE, 0x12, 0x04, 0x95])],
                },
            }],
        }],
        related_formats: &[],
    },
};
