use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975650: FileFormat = FileFormat {
    id: 28_975_650,
    source_type: SourceType::Wikidata,
    name: "Recon Mesh",
    extensions: &["m"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
