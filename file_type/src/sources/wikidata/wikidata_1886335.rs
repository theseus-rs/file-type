use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1886335: FileType = FileType {
    file_format: &FileFormat {
        id: 1_886_335,
        source_type: SourceType::Wikidata,
        name: "Maker Interchange Format",
        extensions: &["mif"],
        media_types: &["application/vnd.mif"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x4D, 0x49, 0x46, 0x46, 0x69, 0x6C, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
