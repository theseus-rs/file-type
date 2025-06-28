use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1343830: FileType = FileType {
    file_format: &FileFormat {
        id: 1_343_830,
        source_type: SourceType::Wikidata,
        name: "Executable and Linkable Format",
        extensions: &["axf", "bin", "elf", "o", "prx", "so"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7F, 0x45, 0x4C, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
