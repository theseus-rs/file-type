use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_136973553: FileType = FileType {
    file_format: &FileFormat {
        id: 136_973_553,
        source_type: SourceType::Wikidata,
        name: "Harvard Montage Album",
        extensions: &["abm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xD6, 0xC5, 0x6F, 0x4D, 0x02, 0x00, 0x40, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
