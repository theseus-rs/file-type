use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110214597: FileFormat = FileFormat {
    id: 110_214_597,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version X8",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
