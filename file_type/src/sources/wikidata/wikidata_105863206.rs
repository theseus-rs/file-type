use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863206: FileFormat = FileFormat {
    id: 105_863_206,
    source_type: SourceType::Wikidata,
    name: "MolMeccano molecule",
    extensions: &["mlm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x4D, 0x6F, 0x6C, 0x4D, 0x65, 0x63, 0x63, 0x61, 0x6E, 0x6F, 0x20,
                    0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20, 0x28, 0x2A, 0x2E, 0x6D, 0x6C, 0x6D,
                    0x29, 0x2E, 0x0D, 0x0A, 0x0D, 0x0A, 0x40,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
