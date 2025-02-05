use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853587: FileFormat = FileFormat {
    id: 105_853_587,
    source_type: SourceType::Wikidata,
    name: "Easy CD Creator Drag to Disk File",
    extensions: &["zl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x78, 0xDA, 0xEC, 0x3B, 0x7F, 0x78])],
            },
        }],
    }],
    related_formats: &[],
};
