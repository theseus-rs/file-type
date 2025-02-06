use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110214597: FileFormat = FileFormat {
    id: 110_214_597,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version X8",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
