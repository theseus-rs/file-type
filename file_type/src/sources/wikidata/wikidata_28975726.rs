use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975726: FileFormat = FileFormat {
    id: 28_975_726,
    source_type: SourceType::Wikidata,
    name: "Gaussian Cube File",
    extensions: &["cub"],
    media_types: &["chemical/x-gaussian-cube"],
    internal_signatures: &[],
    related_formats: &[],
};
