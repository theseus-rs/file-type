use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110039945: FileFormat = FileFormat {
    id: 110_039_945,
    source_type: SourceType::Wikidata,
    name: "Phantom CINE Video File",
    extensions: &["cin", "cine"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
