use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205751: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_751,
        source_type: SourceType::Wikidata,
        name: "BCIF",
        extensions: &["bcif"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x43, 0x49, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
