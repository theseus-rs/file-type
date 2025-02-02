use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_13454995: FileFormat = FileFormat {
    id: 13_454_995,
    source_type: SourceType::Wikidata,
    name: "DVD data file and backup data file",
    extensions: &["bup", "ifo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
