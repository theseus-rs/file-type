use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_73675958: FileFormat = FileFormat {
    id: 73_675_958,
    source_type: SourceType::Wikidata,
    name: "3M Printscape",
    extensions: &["psc", "std"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
