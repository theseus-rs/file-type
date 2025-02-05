use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110213247: FileFormat = FileFormat {
    id: 110_213_247,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version X6",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
