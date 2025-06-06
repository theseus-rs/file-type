use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856935: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_935,
        source_type: SourceType::Wikidata,
        name: "GLF 3D Font File Format",
        extensions: &["glf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x4C, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
