use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_96035181: FileFormat = FileFormat {
    id: 96_035_181,
    source_type: SourceType::Wikidata,
    name: "LEDA",
    extensions: &["gw", "lgr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
