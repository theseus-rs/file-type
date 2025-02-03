use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_72727591: FileFormat = FileFormat {
    id: 72_727_591,
    source_type: SourceType::Wikidata,
    name: "Juno address book",
    extensions: &["nv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
