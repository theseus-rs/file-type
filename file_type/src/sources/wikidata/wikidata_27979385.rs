use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979385: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_385,
        source_type: SourceType::Wikidata,
        name: "Fractal Image And Sequence Codec",
        extensions: &["fco"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x49, 0x41, 0x53, 0x43, 0x4F, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
