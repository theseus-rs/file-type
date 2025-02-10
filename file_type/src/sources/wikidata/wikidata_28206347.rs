use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28206347: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_347,
        source_type: SourceType::Wikidata,
        name: "IndyPaint",
        extensions: &["tru"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x6E, 0x64, 0x79])],
                },
            }],
        }],
        related_formats: &[],
    },
};
