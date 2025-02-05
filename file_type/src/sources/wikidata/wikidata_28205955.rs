use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205955: FileFormat = FileFormat {
    id: 28_205_955,
    source_type: SourceType::Wikidata,
    name: "Dr. Halo Palette",
    extensions: &["pal"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x48, 0xE3, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
