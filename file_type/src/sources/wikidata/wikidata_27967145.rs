use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967145: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_145,
        source_type: SourceType::Wikidata,
        name: "Dual Module Player DSMI",
        extensions: &["amf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x4D, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
