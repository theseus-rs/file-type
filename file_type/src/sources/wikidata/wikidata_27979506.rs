use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979506: FileFormat = FileFormat {
    id: 27_979_506,
    source_type: SourceType::Wikidata,
    name: "Photoshop Transfer Function",
    extensions: &["atf"],
    media_types: &["application/x-photoshop"],
    internal_signatures: &[],
    related_formats: &[],
};
