use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979506: FileFormat = FileFormat {
    id: 27_979_506,
    source_type: SourceType::Wikidata,
    name: "Photoshop Transfer Function",
    extensions: &["atf"],
    media_types: &["application/x-photoshop"],
    signatures: &[],
    related_formats: &[],
};
