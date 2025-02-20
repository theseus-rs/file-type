use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864192: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_192,
        source_type: SourceType::Wikidata,
        name: "Microsoft PhoneBook (UTF-8)",
        extensions: &["pbk"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0x5B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
