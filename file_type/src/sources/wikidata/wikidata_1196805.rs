use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1196805: FileType = FileType {
    file_format: &FileFormat {
        id: 1_196_805,
        source_type: SourceType::Wikidata,
        name: "Resource Interchange File Format",
        extensions: &["riff"],
        media_types: &["application/x-riff"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x49, 0x46, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
