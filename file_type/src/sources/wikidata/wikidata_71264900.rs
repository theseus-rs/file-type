use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71264900: FileType = FileType {
    file_format: &FileFormat {
        id: 71_264_900,
        source_type: SourceType::Wikidata,
        name: "Hemera Photo-Object Image bitmap",
        extensions: &["hpi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x89, 0x48, 0x50, 0x49, 0x0D, 0x0A, 0x1A, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
