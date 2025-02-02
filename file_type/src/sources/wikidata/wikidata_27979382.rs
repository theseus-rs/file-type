use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979382: FileFormat = FileFormat {
    id: 27_979_382,
    source_type: SourceType::Wikidata,
    name: "MPLS",
    extensions: &["mpl", "mpls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
