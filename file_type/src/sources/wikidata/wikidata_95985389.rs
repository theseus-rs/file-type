use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_95985389: FileFormat = FileFormat {
    id: 95_985_389,
    source_type: SourceType::Wikidata,
    name: "Affymetrix CHP file format",
    extensions: &["chp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
