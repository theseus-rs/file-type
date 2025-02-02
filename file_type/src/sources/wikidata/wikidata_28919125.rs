use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28919125: FileFormat = FileFormat {
    id: 28_919_125,
    source_type: SourceType::Wikidata,
    name: "Final Cut Pro X project",
    extensions: &["fcpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
