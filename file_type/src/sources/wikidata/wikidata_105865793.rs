use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865793: FileFormat = FileFormat {
    id: 105_865_793,
    source_type: SourceType::Wikidata,
    name: "Microsoft Windows Installer Patch Creation Properties",
    extensions: &["pcp"],
    media_types: &["application/x-ms-pcp"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
