use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110211459: FileFormat = FileFormat {
    id: 110_211_459,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version X3",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
