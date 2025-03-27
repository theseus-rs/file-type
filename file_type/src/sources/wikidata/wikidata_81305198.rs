use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_81305198: FileType = FileType {
    file_format: &FileFormat {
        id: 81_305_198,
        source_type: SourceType::Wikidata,
        name: "BOA Constrictor Archiver compressed archive",
        extensions: &["boa"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4F, 0x41, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
