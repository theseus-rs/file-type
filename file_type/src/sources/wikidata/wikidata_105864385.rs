use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864385: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_385,
        source_type: SourceType::Wikidata,
        name: "Word Writer 128 print Parameters/options",
        extensions: &["par"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x05, 0x0A, 0x14, 0x1E, 0x28, 0x32, 0x3C, 0x46,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
