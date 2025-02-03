use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113577664: FileFormat = FileFormat {
    id: 113_577_664,
    source_type: SourceType::Wikidata,
    name: "Philips/OptImage's Master tool",
    extensions: &["cd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
