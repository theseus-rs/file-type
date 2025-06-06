use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864775: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_775,
        source_type: SourceType::Wikidata,
        name: "SilkExplorer - Performance Explorer Workspace",
        extensions: &["pex"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0B, 0x00, 0x50, 0x45, 0x57, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
