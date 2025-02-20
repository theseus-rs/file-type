use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865369: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_369,
        source_type: SourceType::Wikidata,
        name: "PatchMeister Driver",
        extensions: &["pmdriver"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4D, 0x44, 0x52, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
