use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119978112: FileFormat = FileFormat {
    id: 119_978_112,
    source_type: SourceType::Wikidata,
    name: "Clip File",
    extensions: &["fmc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
