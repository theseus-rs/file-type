use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73514196: FileFormat = FileFormat {
    id: 73_514_196,
    source_type: SourceType::Wikidata,
    name: "MegaCAD Project",
    extensions: &["prt"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x65, 0x67, 0x61, 0x43, 0x61, 0x64])],
            },
        }],
    }],
    related_formats: &[],
};
