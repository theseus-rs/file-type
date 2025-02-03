use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110216000: FileFormat = FileFormat {
    id: 110_216_000,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication, version 1-3",
    extensions: &["ppp", "ppt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
