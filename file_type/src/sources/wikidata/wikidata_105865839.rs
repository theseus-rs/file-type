use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865839: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_839,
        source_type: SourceType::Wikidata,
        name: "Mobipocket - PRC Palm e-Book",
        extensions: &["mobi", "prc"],
        media_types: &["application/x-mobipocket-ebook"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x4F, 0x4F, 0x4B, 0x4D, 0x4F, 0x42, 0x49,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
