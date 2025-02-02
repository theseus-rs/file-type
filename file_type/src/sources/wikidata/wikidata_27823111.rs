use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27823111: FileFormat = FileFormat {
    id: 27_823_111,
    source_type: SourceType::Wikidata,
    name: "Bathymetry Attributed Grid",
    extensions: &["bag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
