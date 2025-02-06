use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863061: FileFormat = FileFormat {
    id: 105_863_061,
    source_type: SourceType::Wikidata,
    name: "Madonna Model",
    extensions: &["mmd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x4D, 0x61, 0x64, 0x6F, 0x6E, 0x6E, 0x61, 0x4D, 0x6F, 0x64, 0x65, 0x6C,
                    0xF8,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
