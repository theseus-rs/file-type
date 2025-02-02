use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110213247: FileFormat = FileFormat {
    id: 110_213_247,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version X6",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
