use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851228: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_228,
        source_type: SourceType::Wikidata,
        name: "Marmoset Toolbag Material",
        extensions: &["tbmat"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x40, 0x53, 0x75, 0x62, 0x20, 0x53, 0x52, 0x53, 0x75, 0x72, 0x66, 0x61,
                        0x63, 0x65, 0x20, 0x3D, 0x20, 0x53, 0x52, 0x53, 0x75, 0x72, 0x66, 0x61,
                        0x63, 0x65, 0x4E, 0x6F, 0x72, 0x6D, 0x61, 0x6C, 0x4D, 0x61, 0x70,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
