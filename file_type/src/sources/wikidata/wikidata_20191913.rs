use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_20191913: FileFormat = FileFormat {
    id: 20_191_913,
    source_type: SourceType::Wikidata,
    name: "Apple Help File Format",
    extensions: &["lproj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
