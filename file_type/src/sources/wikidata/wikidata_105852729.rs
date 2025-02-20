use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852729: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_729,
        source_type: SourceType::Wikidata,
        name: "Microsoft Serialized certificate Store",
        extensions: &["sst"],
        media_types: &["application/vnd.ms-pki.certstore"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x43, 0x45, 0x52, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
