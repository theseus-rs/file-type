use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63280227: FileFormat = FileFormat {
    id: 63_280_227,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for Windows, version 4.0a",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
