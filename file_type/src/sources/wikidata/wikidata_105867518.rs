use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867518: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_518,
        source_type: SourceType::Wikidata,
        name: "Haines NFF scene",
        extensions: &["nff"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x76])],
                },
            }],
        }],
        related_formats: &[],
    },
};
