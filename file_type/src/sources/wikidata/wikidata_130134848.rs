use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130134848: FileFormat = FileFormat {
    id: 130_134_848,
    puid: "wikidata/130134848",
    name: "Kusto query file",
    extensions: &["kql"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
