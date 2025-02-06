use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852997: FileFormat = FileFormat {
    id: 105_852_997,
    source_type: SourceType::Wikidata,
    name: "EGS-SpectraPaint Stencil",
    extensions: &["stencil"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x53, 0x54, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
