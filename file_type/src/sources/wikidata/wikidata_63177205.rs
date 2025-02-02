use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63177205: FileFormat = FileFormat {
    id: 63_177_205,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for Macintosh, version 4",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
