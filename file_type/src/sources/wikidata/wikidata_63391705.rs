use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63391705: FileFormat = FileFormat {
    id: 63_391_705,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for DOS, version 3b",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
