use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110215455: FileFormat = FileFormat {
    id: 110_215_455,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication, version SE",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
