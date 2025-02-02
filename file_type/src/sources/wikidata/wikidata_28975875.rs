use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975875: FileFormat = FileFormat {
    id: 28_975_875,
    source_type: SourceType::Wikidata,
    name: "OOGL TLIST Projective file",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
