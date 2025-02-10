use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864906: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_906,
        source_type: SourceType::Wikidata,
        name: "Palm MobileDB database",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x4D, 0x64, 0x62, 0x31, 0x4D, 0x64, 0x62, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
