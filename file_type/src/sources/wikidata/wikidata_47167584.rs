use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47167584: FileFormat = FileFormat {
    id: 47_167_584,
    source_type: SourceType::Wikidata,
    name: "ClarisWorks Painting file format",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
