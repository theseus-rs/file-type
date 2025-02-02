use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1617682: FileFormat = FileFormat {
    id: 1_617_682,
    source_type: SourceType::Wikidata,
    name: "DVD BUP File",
    extensions: &["bup"],
    media_types: &["video/dvd"],
    internal_signatures: &[],
    related_formats: &[],
};
