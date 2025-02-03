use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47493509: FileFormat = FileFormat {
    id: 47_493_509,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CS2",
    extensions: &["ind", "indd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
