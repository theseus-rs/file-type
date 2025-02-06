use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110212801: FileFormat = FileFormat {
    id: 110_212_801,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version X5",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
