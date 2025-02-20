use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859034: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_034,
        source_type: SourceType::Wikidata,
        name: "Soldat Bot Information",
        extensions: &["bot"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x42, 0x4F, 0x54, 0x5D, 0x0D, 0x0A, 0x4E, 0x61, 0x6D, 0x65, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
