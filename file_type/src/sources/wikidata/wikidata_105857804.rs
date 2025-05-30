use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857804: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_804,
        source_type: SourceType::Wikidata,
        name: "Virtual APF settings",
        extensions: &["ini"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x4D, 0x45, 0x4D, 0x4F, 0x52, 0x59, 0x5D, 0x0D, 0x0A, 0x63, 0x61,
                        0x72, 0x74, 0x74, 0x79, 0x70, 0x65, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
