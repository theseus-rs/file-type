use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48625328: FileFormat = FileFormat {
    id: 48_625_328,
    source_type: SourceType::Wikidata,
    name: "Encapsulated PostScript, v2",
    extensions: &["eps", "epsf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
