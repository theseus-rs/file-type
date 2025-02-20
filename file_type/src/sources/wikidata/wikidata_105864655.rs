use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864655: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_655,
        source_type: SourceType::Wikidata,
        name: "Microsoft PowerPoint (v3.0)",
        extensions: &["ppt"],
        media_types: &["application/vnd.ms-powerpoint"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xED, 0xDE, 0xAD, 0x0B, 0x03, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
