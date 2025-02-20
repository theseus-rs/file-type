use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864746: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_746,
        source_type: SourceType::Wikidata,
        name: "Palm WineMaster list",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x50, 0x63, 0x57, 0x62, 0x50, 0x62, 0x57, 0x6E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
