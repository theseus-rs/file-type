use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130543129: FileFormat = FileFormat {
    id: 130_543_129,
    source_type: SourceType::Wikidata,
    name: "Puppet configuration file format",
    extensions: &["pp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
