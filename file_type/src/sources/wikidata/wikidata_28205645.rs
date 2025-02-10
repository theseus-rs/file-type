use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28205645: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_645,
        source_type: SourceType::Wikidata,
        name: "XV thumbnail",
        extensions: &["p7"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x37, 0x20, 0x33, 0x33, 0x32, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
