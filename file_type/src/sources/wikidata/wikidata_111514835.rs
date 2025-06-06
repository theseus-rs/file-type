use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111514835: FileType = FileType {
    file_format: &FileFormat {
        id: 111_514_835,
        source_type: SourceType::Wikidata,
        name: "3D Movie Maker",
        extensions: &["3mm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x48, 0x4E, 0x32, 0x20, 0x43, 0x4F, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
