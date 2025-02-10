use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855977: FileFormat = FileFormat {
    id: 105_855_977,
    source_type: SourceType::Wikidata,
    name: "COMit modems configuration",
    extensions: &["dat", "dos"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4F, 0x44, 0x45, 0x4D, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
