use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205485: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_485,
        source_type: SourceType::Wikidata,
        name: "AMOS Icon Bank",
        extensions: &["abk"],
        media_types: &["application/octet-stream", "image/x-amos-iconbank"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x6D, 0x49, 0x63])],
                },
            }],
        }],
        related_formats: &[],
    },
};
