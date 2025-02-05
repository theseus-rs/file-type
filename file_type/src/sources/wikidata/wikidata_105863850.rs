use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863850: FileFormat = FileFormat {
    id: 105_863_850,
    source_type: SourceType::Wikidata,
    name: "Cumulate Draw's editable MMD format",
    extensions: &["mmd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
