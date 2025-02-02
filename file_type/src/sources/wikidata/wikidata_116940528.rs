use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116940528: FileFormat = FileFormat {
    id: 116_940_528,
    source_type: SourceType::Wikidata,
    name: "AccessData Recovery Session",
    extensions: &["adr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
