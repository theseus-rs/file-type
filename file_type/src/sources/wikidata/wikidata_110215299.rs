use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110215299: FileFormat = FileFormat {
    id: 110_215_299,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version X9",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
