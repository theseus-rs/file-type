use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1437034: FileType = FileType {
    file_format: &FileFormat {
        id: 1_437_034,
        source_type: SourceType::Wikidata,
        name: "Photoshop Big",
        extensions: &["psb"],
        media_types: &["image/vnd.adobe.photoshop"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x38, 0x42, 0x50, 0x53, 0x00, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
