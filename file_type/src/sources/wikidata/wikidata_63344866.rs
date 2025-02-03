use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63344866: FileFormat = FileFormat {
    id: 63_344_866,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for Windows, version 2000",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
