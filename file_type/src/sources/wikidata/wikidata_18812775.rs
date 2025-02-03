use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_18812775: FileFormat = FileFormat {
    id: 18_812_775,
    source_type: SourceType::Wikidata,
    name: "VTK format",
    extensions: &["vtk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
