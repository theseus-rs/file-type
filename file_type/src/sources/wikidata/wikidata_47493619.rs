use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47493619: FileFormat = FileFormat {
    id: 47_493_619,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CS4",
    extensions: &["ind", "indd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
