use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853562: FileFormat = FileFormat {
    id: 105_853_562,
    source_type: SourceType::Wikidata,
    name: "Zen Guard license",
    extensions: &["zl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x72, 0x6F, 0x64, 0x75, 0x63, 0x74, 0x2D, 0x4E, 0x61, 0x6D, 0x65, 0x20,
                    0x3D, 0x20, 0x5A, 0x65, 0x6E, 0x64, 0x20, 0x47, 0x75, 0x61, 0x72, 0x64,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
