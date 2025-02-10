use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857463: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_463,
        source_type: SourceType::Wikidata,
        name: "Cyber Studio CAD-3D v2 object",
        extensions: &["3d2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3D, 0x02, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
