use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864937: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_937,
        source_type: SourceType::Wikidata,
        name: "PyScripter Project (UTF)",
        extensions: &["psproj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x5B, 0x50, 0x79, 0x53, 0x63, 0x72, 0x69, 0x70, 0x74,
                        0x65, 0x72, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
