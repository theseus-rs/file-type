use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47493628: FileFormat = FileFormat {
    id: 47_493_628,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CS5",
    extensions: &["ind", "indd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
