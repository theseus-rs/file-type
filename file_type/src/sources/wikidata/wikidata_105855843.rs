use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855843: FileFormat = FileFormat {
    id: 105_855_843,
    source_type: SourceType::Wikidata,
    name: "AutoCAD R14 Drawing",
    extensions: &["dwg"],
    media_types: &["application/x-autocad"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x31, 0x30, 0x31, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
