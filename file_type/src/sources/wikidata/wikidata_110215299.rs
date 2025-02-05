use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110215299: FileFormat = FileFormat {
    id: 110_215_299,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version X9",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
