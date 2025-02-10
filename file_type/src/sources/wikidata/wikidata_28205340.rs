use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28205340: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_340,
        source_type: SourceType::Wikidata,
        name: "Hasselblad 3F RAW",
        extensions: &["3fr"],
        media_types: &["image/x-raw-hasselblad"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x49, 0x2A, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
