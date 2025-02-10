use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855934: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_934,
        source_type: SourceType::Wikidata,
        name: "MSX-DOS disk Image",
        extensions: &["dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xEB, 0xFE, 0x90])],
                },
            }],
        }],
        related_formats: &[],
    },
};
