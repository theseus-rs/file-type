use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110985792: FileFormat = FileFormat {
    id: 110_985_792,
    source_type: SourceType::Wikidata,
    name: "Twin VQ format",
    extensions: &["vqf"],
    media_types: &["audio/x-twinvq"],
    signatures: &[],
    related_formats: &[],
};
