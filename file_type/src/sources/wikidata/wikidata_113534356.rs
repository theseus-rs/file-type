use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113534356: FileFormat = FileFormat {
    id: 113_534_356,
    source_type: SourceType::Wikidata,
    name: "Pro Tools Session File",
    extensions: &["ptx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
