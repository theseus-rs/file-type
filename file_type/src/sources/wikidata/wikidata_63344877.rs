use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63344877: FileFormat = FileFormat {
    id: 63_344_877,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for DOS",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
