use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110211790: FileFormat = FileFormat {
    id: 110_211_790,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version X4",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
