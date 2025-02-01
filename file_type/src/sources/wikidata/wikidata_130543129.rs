use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130543129: FileFormat = FileFormat {
    id: 130_543_129,
    puid: "wikidata/130543129",
    name: "Puppet configuration file format",
    extensions: &["pp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
