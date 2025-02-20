use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850804: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_804,
        source_type: SourceType::Wikidata,
        name: "Google Earth import definition",
        extensions: &["kdx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x61, 0x79, 0x6F, 0x75, 0x74, 0x20, 0x7B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
