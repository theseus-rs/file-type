use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130619766: FileFormat = FileFormat {
    id: 130_619_766,
    puid: "wikidata/130619766",
    name: "RelaxNG compact syntax file format",
    extensions: &["rnc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
