use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_360194: FileFormat = FileFormat {
    id: 360_194,
    puid: "wikidata/360194",
    name: "Adobe Type 1 ASCII Font Metrics",
    extensions: &["afm"],
    media_types: &["application/x-font-afm"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x6F, 0x6E, 0x74, 0x4D, 0x65, 0x74, 0x72,
                    0x69, 0x63, 0x73, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
