use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110213914: FileFormat = FileFormat {
    id: 110_213_914,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version X7",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
