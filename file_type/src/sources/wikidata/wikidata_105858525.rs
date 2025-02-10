use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858525: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_525,
        source_type: SourceType::Wikidata,
        name: "IBM Softcopy Reader (Bookmanager) book file",
        extensions: &["bks"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x4B, 0x53, 0x48, 0x45, 0x4C, 0x46, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
