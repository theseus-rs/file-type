use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975865: FileFormat = FileFormat {
    id: 28_975_865,
    source_type: SourceType::Wikidata,
    name: "OOGL VECT file",
    extensions: &["vect"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
